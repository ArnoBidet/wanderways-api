use rocket::serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GameMetadata {
    pub id_map: String,
    pub id_gamemod: String,
    pub lang: String
}