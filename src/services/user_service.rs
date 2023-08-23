use reqwest::Client;
use base64::{engine::general_purpose, Engine as _};

use crate::models::user::{User, PlayerData, ProfileData, DecodedProfileData, UserTexture};

pub async fn get_user_info(username: String) -> Option<User> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);

    let client = Client::new();
    let res = client.get(&url)
    .send()
    .await;

    match res {
        Ok(res) => {
            let body = res.text().await.unwrap();

            let player_data: PlayerData = serde_json::from_str(&body).unwrap();
            let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", player_data.id);
            
            let res = client.get(&url)
                .send()
                .await;
            
            let body = res.unwrap().text().await.unwrap();
            let data = get_data(body);

            let user = User {
                uuid: data.profile_id,
                username: data.profile_name,
                textures: UserTexture {
                    cape: data.textures.cape.url,
                    skin: data.textures.skin.url,
                },
                time: data.timestamp,
            };

            Some(user)

        },
        Err(_) => None
    }

}


fn get_data(body: String) -> DecodedProfileData {
    let json: ProfileData = serde_json::from_str(&body).unwrap();

    let code: &str = json.properties[0].value.as_str();
    let decode = general_purpose::STANDARD.decode(code.as_bytes()).unwrap();
    
    let str = String::from_utf8(decode).unwrap();
    println!("{}", str);
    let data: DecodedProfileData = serde_json::from_str(&str).unwrap();

    data
}