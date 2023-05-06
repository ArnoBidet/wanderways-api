use chrono::{DateTime, Utc};

#[derive(Default, Clone, Debug)]
pub struct SessionData {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String,
    pub expiration_time: DateTime<Utc>,
    pub remaining: Vec<SessionGeoData>, //@TODO: Use reference instead a copy
    pub answers: Vec<SessionGeoData>
}

#[derive(Default, Clone, Debug)]
pub struct SessionGeoData{
    pub id : String,
    pub translations : Vec<String>
}