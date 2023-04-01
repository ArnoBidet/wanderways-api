use rocket::serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct GeoData {
    pub id: String,
    pub label: Vec<String>,
    pub group: Option<GeoDataItem>,
    pub capital: Option<GeoDataItem>,
    pub numeric_code: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct GeoDataItem {
    pub id: String,
    pub label: Vec<String>,
}