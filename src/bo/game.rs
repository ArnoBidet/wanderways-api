use rocket::serde::Serialize;

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Debug, PartialEq)]
pub struct Game {
    pub id: String,
    pub play_count: i32,
}