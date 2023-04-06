use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct AverageAwareness {
    pub play_count: i32,
    pub data: Vec<AverageAwarenessItem>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct AverageAwarenessItem {
    pub id: String,
    pub found_count: i32,
}
