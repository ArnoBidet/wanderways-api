use rocket::serde::json::Json;

use crate::bll::game_list::game_list;
use crate::bo::game::Game;
use crate::PgDatabase;

// @TODO deal with error 500
#[get("/game/list")]
pub async fn get_game_list(conn: PgDatabase) -> Json<Vec<Game>> {
    conn.run(move |client| game_list(client)).await.unwrap()
}
