use rocket::serde::json::Json;
use rocket::http::Status;

use chrono::{ DateTime, Duration as ChronoDuration, Utc };

use crate::bo::session_data::{ SessionData, SessionGeoData };
use crate::bo::game_party::GameParty;

use crate::bll::start_game::start_game as bll_start_game;
use crate::bo::start_game::StartGame;

pub type Session<'a> = rocket_session::Session<'a, SessionData>;

const AVERAGE_RESPONSE_DURATION: i64 = 10;
const LATENCY_IN_SECONDS: i64 = 30;

#[post("/start", data = "<game_party>")]
pub async fn start_game<'a>(session: Session<'a>, game_party: Json<GameParty>) -> Result<Json<Vec<StartGame>>, Status> {
    let game: GameParty = game_party.into_inner();
    let game_lang_slice: &str = &game.lang[..];
    let game_map_slice: &str = &game.id_map[..];

    let game_answers: Vec<SessionGeoData> = match bll_start_game(game_lang_slice, game_map_slice).await {
        Ok(answers) => answers,
        Err(_) => return Err(Status::BadRequest)
    };

    let game_answers_copy = game_answers.clone();

    let now: DateTime<Utc> = Utc::now();
    let game_duration = game_answers.len() as i64 * AVERAGE_RESPONSE_DURATION;
    let duration = ChronoDuration::seconds(game_duration + LATENCY_IN_SECONDS);

    let session_time_stamp = now.timestamp();

    session.tap( |sess: &mut SessionData| {
        sess.id_map = game.id_map;
        sess.id_gamemode = game.id_gamemode;
        sess.lang = game.lang;
        sess.expiration_time = now + duration;
        sess.remaining = game_answers_copy.clone();
        sess.answers = game_answers_copy;
    });

    println!("{:?}", session);

    let mut result: Vec<StartGame> = vec![];

    result.push(StartGame {
        start_session_timestamp: session_time_stamp,
        session_duration: duration.num_seconds(),
    });
    
    Ok(Json(result))
}