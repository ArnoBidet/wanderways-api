use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Map {
    pub id_map: String,
    pub map_label : Vec<String>,
    pub tags : Vec<String>,
    pub map_description : Vec<String>,
    pub url_wiki : String,
    pub play_count : i32
}