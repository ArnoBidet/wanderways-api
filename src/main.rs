#[macro_use]
extern crate rocket;
use std::env;

use dotenvy::dotenv;
use rocket::launch;
use routes::average_awareness::get_average_awareness;
use routes::game_list::get_game_list;
use routes::get_map_geo_data::get_map_geo_data;
use routes::map_list::get_map_list;
use routes::tag_list::get_tag_list;

mod translation_parser;

mod bll {
    pub mod average_awareness;
    pub mod game_list;
    pub mod map_geo_data;
    pub mod map_list;
    pub mod tag_list;
}
mod bo {
    pub mod average_awareness;
    pub mod game;
    pub mod geo_data;
    pub mod map;
    pub mod tag;
}
mod dal {
    pub mod average_awareness;
    pub mod establish_connection;
    pub mod game_list;
    pub mod map_geo_data;
    pub mod map_list;
    pub mod tag_list;
    pub mod query;
}
mod routes {
    pub mod utils {
        pub mod lang_utils;
    }
    pub mod average_awareness;
    pub mod game_list;
    pub mod get_map_geo_data;
    pub mod guards;
    pub mod map_list;
    pub mod responders;
    pub mod tag_list;
}

#[launch]
fn rocket() -> _ {
    let app_env : String = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    if app_env == "development"{ // if in dev mod, load .env file, else env var are based on execution context
        dotenv().expect(".env file not found");
    }
    // @TODO extract to another file ?
    // Add support for 404 error
    rocket::build().mount(
        "/",
        routes![
    get_game_list,
    get_map_list,
    get_tag_list,
    get_map_geo_data,
    get_average_awareness],
    )
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

    #[test]
    fn get_average_awareness() {
        let client = get_client();
        let response = client
            .get(format!(
                "/average-awareness/{}/{}?id_lang={}",
                "FRANCE_DEPARTMENTS", "AGAINST_CLOCK", "fr-FR"
            ))
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

        fn get_tag_list() {
            let client = get_client();
            let response = client
                .get("/tag/list")
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
}
