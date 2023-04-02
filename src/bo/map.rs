use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub id: String,
    pub label : String,
    pub tags : Vec<String>,
    pub description : String,
    pub url_wiki : String,
    pub play_count : i32
}