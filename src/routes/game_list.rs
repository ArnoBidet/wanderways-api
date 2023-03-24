use rocket::serde::json::Json;

use crate::bll::game_list::game_list;
use crate::bo::game::Game;

// @TODO deal with error 500
#[get("/game/list")]
pub async fn get_game_list() -> Json<Vec<Game>> {
    game_list().await.unwrap()
}
