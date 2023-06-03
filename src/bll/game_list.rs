use crate::{bo::game::Game, dal::game_list::game_list as dal_game_list, PgDatabase};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

pub async fn game_list(client: &Connection<PgDatabase>) -> Result<Json<Vec<Game>>, Error> {
    dal_game_list(client).await.and_then(|res| Ok(Json(res)))
}
