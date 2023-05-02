use rocket::serde::json::Json;
use rocket::http::Status;

use rocket::http::{ Cookie, CookieJar };

use chrono::{ DateTime, Duration as ChronoDuration, Utc };

use crate::bo::session_data::{ SessionData, SessionGeoData };
use crate::bo::game_party::GameParty;

use crate::bll::start_game::start_game as bll_start_game;
use crate::bo::start_game::StartGame;

use std::collections::HashMap;

pub type Session<'a> = rocket_session::Session<'a, SessionData>;

#[post("/start", data = "<game_party>")]
pub async fn start_game<'a>(session: Session<'a>, cookies: &CookieJar<'a>, game_party: Json<GameParty>) -> Result<Json<Vec<StartGame>>, Status> {
    let game: GameParty = game_party.into_inner();
    let game_lang_slice: &str = &game.lang[..];
    let game_gamemode_slice: &str = &game.id_gamemode[..];

    let game_answers: HashMap<std::string::String, SessionGeoData> = match bll_start_game(game_lang_slice, game_gamemode_slice).await {
        Ok(answers) => answers,
        Err(_) => return Err(Status::BadRequest)
    };

    let game_answers_copy = game_answers.clone();

    let now: DateTime<Utc> = Utc::now();
    let fake_seconds_to_add = 1000;
    let latence_in_seconds = 30;
    let duration = ChronoDuration::seconds(fake_seconds_to_add + latence_in_seconds);

    let session_time_stamp = now.timestamp();

    println!("{}", session_time_stamp);

    session.tap( |sess: &mut SessionData| {
        sess.id_map = game.id_map;
        sess.id_gamemode = game.id_gamemode;
        sess.lang = game.lang;
        sess.expiration_time = now + duration;
        sess.remaining = HashMap::new();
        sess.answers = game_answers_copy;
    });

    // Création d'un cookie de session avec l'option HTTP Only
    let cookie = Cookie::build("my_session", session_time_stamp.to_string())
        .http_only(true)
        // .secure(true)
        .finish();

    // Ajout du cookie à la réponse HTTP
    cookies.add(cookie);

    let mut result: Vec<StartGame> = vec![];

    result.push(StartGame {
        start_session_timestamp: session_time_stamp,
        session_duration: duration.num_seconds(),
    });
    
    Ok(Json(result))
}