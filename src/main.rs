#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use cors::CORS;
use rocket::{
    figment::{
        util::map,
        value::{Map, Value},
    },
    launch,
};
use crate::env_load::env_parser;
use rocket_db_pools::{Database, deadpool_postgres};
use routes::{
    average_awareness::get_average_awareness,
    game_list::get_game_list,
    get_map_geo_data::get_map_geo_data,
    map_list::get_map_list,
    start_game::{start_game, Session as custom_session},
    tag_list::get_tag_list,
};
use std::env;

mod translation_parser;
mod env_load;
mod cors;
mod bll {
    pub mod average_awareness;
    pub mod game_list;
    pub mod map_geo_data;
    pub mod map_list;
    pub mod start_game;
    pub mod tag_list;
}
mod bo {
    pub mod average_awareness;
    pub mod game;
    pub mod game_metadata;
    pub mod geo_data;
    pub mod map;
    pub mod session_data;
    pub mod start_game;
    pub mod tag;
}
mod dal {
    pub mod average_awareness;
    pub mod game_list;
    pub mod map_geo_data;
    pub mod map_list;
    pub mod query;
    pub mod start_game;
    pub mod tag_list;
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
    pub mod start_game;
    pub mod tag_list;
}

#[derive(Database)]
#[database("wanderways_db")]
pub struct PgDatabase(deadpool_postgres::Pool);

#[launch]
fn rocket() -> _ {
    let rocket_env: String = env::var("ROCKET_ENV").unwrap_or_else(|_| "production".to_string());
    if rocket_env != "production" {
        // if in dev mod, load .env file, else env var are based on execution context
        dotenv().expect(".env file not found");
    }
    let env_vars = env_parser();
    println!("{}", env_vars.db_host);
    let db: Map<_, Value> = map! {
    "url" => format!("postgres://{}:{}@localhost/{}?host={}",
        env_vars.db_user,
        env_vars.db_pass,
        env_vars.db_name,
        env_vars.db_host,
    ).into()};

    let figment = rocket::Config::figment().merge(("databases", map!["wanderways_db" => db]));

    // @TODO Add support for 404 error
    rocket::custom(figment)
        .mount(
            "/",
            routes![
                get_game_list,
                get_map_list,
                get_tag_list,
                get_map_geo_data,
                get_average_awareness,
                start_game
            ],
        )
        .attach(PgDatabase::init())
        .attach(custom_session::fairing())
        .attach(CORS)
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
    }
    #[test]
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
