use crate::{bo::game::Game, dal::game_list::game_list as dal_game_list};
use rocket::serde::json::Json;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::Error;

pub fn game_list(client: &mut Client) -> Result<Json<Vec<Game>>, Error> {
    dal_game_list(client).and_then(|res| Ok(Json(res)))
}
