use chrono::{DateTime, Utc};

use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct SessionData {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String,
    pub expiration_time: DateTime<Utc>,
    pub remaining: HashMap<std::string::String, SessionGeoData>, //@TODO: Use reference instead a copy
    pub answers: HashMap<std::string::String, SessionGeoData>
}

#[derive(Default, Clone, Debug)]
pub struct SessionGeoData{
    pub id : String,
    pub translations : Vec<String>
}