#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::launch;
use routes::game_list::get_game_list;
use routes::get_map_geo_data::get_map_geo_data;
use routes::map_list::get_map_list;

mod translation_parser;

mod bll {
    pub mod game_list;
    pub mod map_list;
    pub mod map_geo_data;
}
mod bo {
    pub mod game;
    pub mod map;
    pub mod geo_data;
}
mod dal {
    pub mod establish_connection;
    pub mod game_list;
    pub mod map_list;
    pub mod map_geo_data;
    pub mod query;
}
mod routes {
    pub mod game_list;
    pub mod get_map_geo_data;
    pub mod guards;
    pub mod map_list;
    pub mod responders;
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    // @TODO extract to another file ?
    // Add support for 404 error
    rocket::build().mount("/", routes![get_game_list, get_map_list, get_map_geo_data])
}

#[cfg(test)]
mod main_tests {
    use super::rocket;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    fn get_client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    #[test]
    fn rocket_launches() {
        let client = Client::tracked(rocket());
        assert!(match client {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn get_game_list() {
        let client = get_client();
        let response = client.get("/game/list").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_map_list() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/map/list").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.headers().get_one("Content-Language").unwrap(),
            "en-US"
        );
    }
    #[test]
    fn get_map_list_fr() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/map/list")
            .header(Header::new("Accept-Language", "fr-FR"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.headers().get_one("Content-Language").unwrap(),
            "fr-FR"
        );
    }

    #[test]
    fn get_map_data_for_map() {
        let client = get_client();
        let response = client
            .get(format!("/map-geo-data/{}", "FRANCE_DEPARTMENTS"))
            .header(Header::new("Accept-Language", "fr-FR"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(
            response.headers().get_one("content-type").unwrap(),
            "application/json"
        );
        assert_eq!(
            response.headers().get_one("Content-Language").unwrap(),
            "fr-FR"
        );
    }
}
