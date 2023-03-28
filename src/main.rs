#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;
use routes::map_list::get_map_list;

mod translation_parser;

mod bll {
    pub mod game_list;
    pub mod map_list;
}
mod bo {
    pub mod game;
    pub mod map;
}
mod dal {
    pub mod establish_connection;
    pub mod game_list;
    pub mod map_list;
}
mod routes {
    pub mod game_list;
    pub mod map_list;
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    // @TODO extract to another file ?
    // Add support for 404 error
    rocket::build().mount("/", routes![get_game_list, get_map_list])
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
