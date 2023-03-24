use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: String,
    pub play_count: i32,
}