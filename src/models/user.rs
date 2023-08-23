use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub struct Textures {
    pub cape : String,
    pub skin : String
}

impl fmt::Display for Textures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cape: {}, skin: {}", self.cape, self.skin)
    }
}

#[derive(Serialize)]
pub struct User {
    pub uuid : String,
    pub username : String,
    pub textures : Textures,
    pub time: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uuid: {}, username: {}, textures: {}, time: {}", self.uuid, self.username, self.textures, self.time)
    }
}