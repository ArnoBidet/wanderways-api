use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{
    bo::game::Game,
    dal::game_party_answer::game_party_answer as dal_game_party_answer
};

pub async fn game_answer_list() -> Result<Vec<String>, Error> {
    dal_game_party_answer().await.and_then(|res| Ok(Vec(res)))
}
