#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;
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
    rocket::build().mount("/", routes![get_game_list])
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
>>>>>>> origin/main
}

#[derive(Debug, Deserialize)]
struct Game {
    id_map: String,
    id_gamemode: String,
    lang: String
}

#[post("/start", data = "<game>")]
async fn start_game(game: Json<Game>) -> Result<String, Status> {

    // Vérifier si map, gamemode et langue existe en BDD
    let checkIfGameExists = "[INSERT SQL REQUEST HERE]";

    if(checkIfGameExists != "")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/gamemode", routes![start_game])
}