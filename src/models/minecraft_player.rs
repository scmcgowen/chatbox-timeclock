use serde::{Deserialize, Serialize};

static API_ENDPOINT: &str = "https://api.mojang.com/users/profiles/minecraft/";


#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftPlayer {
    pub id: String,
    pub name: String,
}

impl MinecraftPlayer {
    pub async fn from_mojang(username: &str) -> Result<Self, reqwest::Error> {
        tracing::info!("Fetching player data from Mojang API, url {}", format!("{}{}", API_ENDPOINT, username));
        let response = reqwest::get(format!("{}{}", API_ENDPOINT, username)).await?;
        response.json::<MinecraftPlayer>().await

    }
}
