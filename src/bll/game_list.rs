use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{bo::game::Game, dal::game_list::game_list as dal_game_list};

pub async fn game_list() -> Result<Json<Vec<Game>>, Error> {
    dal_game_list().await.and_then(|res| Ok(Json(res)))
}
