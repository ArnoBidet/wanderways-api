#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, json::Json};
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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