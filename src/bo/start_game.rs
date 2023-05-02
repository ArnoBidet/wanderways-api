use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StartGame {
    pub start_session_timestamp: i64,
    pub session_duration: i64
}