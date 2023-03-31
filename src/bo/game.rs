use rocket::serde::Serialize;

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Debug, PartialEq)]
pub struct Game {
    pub id_gamemod: String,
    pub play_count: i32,
}