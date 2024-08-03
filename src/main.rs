use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, join};

#[derive(Serialize, Deserialize, Debug)]
struct StickerList {
    pub id: String,
    pub name: String,
    pub tags: String,
    pub r#type: i64,
    pub format_type: i64,
    pub description: String,
    pub asset: String,
    pub pack_id: String,
    pub sort_value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct StickerGroup {
    pub id: String,
    pub sku_id: String,
    pub name: String,
    pub description: String,
    pub stickers: Vec<StickerList>,
    pub cover_sticker_id: String,
    pub banner_asset_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    pub sticker_packs: Vec<StickerGroup>,
}

impl Root {
    async fn get() -> Result<Root, Box<dyn std::error::Error>> {
        let res = reqwest::get("https://discord.com/api/v10/sticker-packs").await?;
        let body = res.text().await?;
        let root: Root = serde_json::from_str(&body)?;
        Ok(root)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Root::get().await?;
    let mut file = File::create("sticker_packs.json").await?;
    file.write_all(serde_json::to_string_pretty(&a)?.as_bytes()).await?;
    println!("Done!");
    Ok(())
}
