use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id_gamemod: String,
    pub play_count: i32,
}