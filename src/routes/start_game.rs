use rocket::serde::json::Json;
use rocket::http::Status;

use chrono::{ DateTime, Duration as ChronoDuration, Utc };

use crate::bo::session_data::SessionData;
use crate::bo::game_party::GameParty;

use crate::bll::game_answer_list::game_answer_list;

type Session<'a> = rocket_session::Session<'a, SessionData>;

#[post("/start", data = "<game_party>")]
pub async fn start_game(session: Session<'_>, game_party: Json<GameParty>) -> Result<String, Status> {

    // let game_answers: Vec<String> = vec![];
    let game_answers: Vec<String> = game_answer_list();

    if game_answers.is_empty()
    {
        // Ajouter les infos dans la session (idMap, idGamemode, arrayUserGoodGuesses, arrayGoodAnswers)

        let game = game_party.into_inner();

        let now: DateTime<Utc> = Utc::now();
        let fake_seconds_to_add = 1000;
        let latence_in_seconds = 30;

        let duration = ChronoDuration::seconds(fake_seconds_to_add + latence_in_seconds);

        // Ajouter la date de fin dans la session, celle-ci sera vérifier lors des requêtes suivantes et supprimer via session.clear si la date est supérieur.
        session.tap(|sess| {

            sess.id_map = game.id_map;
            sess.id_gamemode = game.id_gamemode;
            sess.lang = game.lang;
            sess.expiration_time = now + duration;
            sess.user_answers = vec![];
            sess.good_answers = game_answers
        });

        Ok(format!("print test {:?}", "ça marche cplc."))

    } else {

        // On renvoie une erreur 500
        Err(Status::BadRequest)
    }
}