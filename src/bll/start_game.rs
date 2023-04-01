use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{
    bo::game::Game,
    dal::game_list::game_list as dal_game_list
};

pub async fn start_game() -> Result<[String], Error> {
    dal_game_list().await.and_then(|res| Ok(Json(res)))
}
