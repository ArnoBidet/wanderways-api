use rocket::serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GameParty {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String
}