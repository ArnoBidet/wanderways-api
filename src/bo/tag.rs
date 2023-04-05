use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct TagGroup {
    pub id: String,
    pub label: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Tag {
    pub id: String,
    pub label: String,
}