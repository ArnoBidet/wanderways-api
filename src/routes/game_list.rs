use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::bll::game_list::game_list;
use crate::bo::game::Game;
use crate::PgDatabase;

// @TODO deal with error 500
#[get("/game/list")]
pub async fn get_game_list(client: Connection<PgDatabase>) -> Json<Vec<Game>> {
    game_list(&client).await.unwrap()
}
