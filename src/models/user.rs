use serde::{Serialize, Deserialize};#[derive(Serialize)]

pub struct User {
    pub uuid: String,
    pub username: String,
    pub textures: UserTexture,
    pub time: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTexture {
    pub cape: Option<Cape>,
    pub skin: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileData {
    pub id: String,
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodedProfileData {
    pub timestamp: i64,
    pub profile_id: String,
    pub profile_name: String,
    pub textures: Textures,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Textures {
    pub skin: Cape,
    pub cape: Option<Cape>,
}

#[derive(Serialize, Deserialize)]
pub struct Cape {
    pub url: String,
}
