#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;

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
}
