use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{
    bo::game::Game,
    dal::start_game::start_game as dal_start_game
};

pub async fn start_game() -> Result<Vec<String>, Error> {
    dal_start_game().await.and_then(|res| Ok(Vec(res)))
}
