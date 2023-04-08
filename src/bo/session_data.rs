use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
pub struct SessionData<'a> {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String,
    pub expiration_time: DateTime<Utc>,
    pub found: Vec<&'a GeoData>,
    pub remaining: Vec<&'a GeoData>,
    pub answers: Vec<GeoData>
}

#[derive(Default, Clone)]
struct GeoData{
    id : String,
    translations : Vec<String>
}