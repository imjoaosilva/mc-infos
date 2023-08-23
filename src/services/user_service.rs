use reqwest::Client;
use base64::{engine::general_purpose, Engine as _};

use crate::models::user::{User, Textures};

pub async fn get_user_info(username: String) -> Option<User> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);

    let client = Client::new();
    let res = client.get(&url)
    .send()
    .await;

    match res {
        Ok(res) => {
            let body = res.text().await.unwrap();
            let json: serde_json::Value = serde_json::from_str(&body).unwrap();
            let uuid = json["id"].to_string().replace("\"", "");
            let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", uuid);
            
            let res = client.get(&url)
                .send()
                .await;
            
            let body = res.unwrap().text().await.unwrap();
            let json: serde_json::Value = serde_json::from_str(&body).unwrap();
            let decode = json["properties"][0]["value"].to_string().replace("\"", "");
            let result = general_purpose::STANDARD.decode(decode.as_bytes()).unwrap();
            let str = String::from_utf8(result).unwrap();
            let data: serde_json::Value = serde_json::from_str(&str).unwrap();

            let user = User {
                uuid: data["profileId"].to_string(),
                username: data["profileName"].to_string(),
                textures: Textures {
                    cape: data["textures"]["CAPE"]["url"].to_string(),
                    skin: data["textures"]["SKIN"]["url"].to_string(),
                },
                time: data["timestamp"].to_string(),
            };

            Some(user)
        },
        Err(_) => None
    }

}