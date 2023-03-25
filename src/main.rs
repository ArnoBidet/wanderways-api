#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;

type Session<'a> = rocket_session::Session<'a, u64>;

use std::time::Duration;

use rocket::serde::{Deserialize, json::Json};
use rocket::http::Status;

mod bll {
    pub mod game_list;
}
mod bo {
    pub mod game;
}
mod dal {
    pub mod establish_connection;
    pub mod game_list;
}
mod routes {
    pub mod game_list;
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    // @TODO extract to another file ?
    // Add support for 404 error
    rocket::build()
        .mount("/", routes![get_game_list])
        .mount("/test", routes![index_test])
        .attach(Session::fairing().with_lifetime(Duration::from_secs(1000)))
        .mount("/api/gamemode", routes![start_game])
}

#[cfg(test)]
mod main_tests {
    use super::rocket;
    use rocket::local::blocking::Client;

    #[test]
    fn rocket_launches() {
        let client = Client::tracked(rocket());
        assert!(match client {
            Ok(_) => true,
            Err(_) => false,
        });
    }
}

#[derive(Debug, Deserialize)]
struct Game {
    id_map: String,
    id_gamemode: String,
    lang: String
}

#[get("/session")]
fn index_test(session: Session) -> String {
    let count = session.tap(|n| {
        // Change the stored value (it is &mut)
        *n += 1;

        // Return something to the caller.
        // This can be any type, 'tap' is generic.
        *n
    });

    format!("{} visits", count)
}

#[post("/start", data = "<game>")]
async fn start_game(session: Session<'_>, game: Json<Game>) -> Result<String, Status> {

    // Ajouter la date de fin dans la session, celle-ci sera vérifier lors des requêtes suivantes et supprimer via session.clear si la date est supérieur.
    let count = session.tap(|n| {
        *n += 1;

        *n
    });

    format!("{} visits", count);

    // Vérifier si map, gamemode et langue existe en BDD
    // let checkIfGameExists = "[INSERT SQL REQUEST HERE]";
    let check_if_game_exists = "pwet";

    if check_if_game_exists != "pwet"
    {
        // Créer une session

        // Ajouter les infos dans la session (idMap, idGamemode, arrayUserGoodGuesses, arrayGoodAnswers)

        // Créer un cookie avec en durée le temps de partie + 10 sec

        // Ajouter l'id de la session dans le cookie
        Ok(format!("print test {:?}", game))

    } else {

        // On renvoie une erreur 500
        Err(Status::BadRequest)
    }

}