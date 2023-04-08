use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
pub struct SessionData {
    pub id_map: String,
    pub id_gamemode: String,
    pub lang: String,
    pub expiration_time: DateTime<Utc>,
    pub user_answers: Vec<String>,
    pub good_answers: Vec<String>
}