use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
pub struct SessionData {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String,
    pub expiration_time: DateTime<Utc>,
    pub found: Vec<&GeoData>,
    pub remaining: Vec<&GeoData>,
    pub answers: Vec<GeoData>
}

#[derive(Default)]
struct GeoData{
    id :string,
    translations : Vec<string>
}