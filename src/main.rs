#[macro_use] extern  crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;

mod dal;
mod bo;
mod bll;
mod routes;

#[launch]
async fn rocket() -> _ {
    dotenv().expect(".env file not found");
    // @TODO extract to another file ?
    // Add support for 404 error
    rocket::build().mount("/",routes![get_game_list])
}
