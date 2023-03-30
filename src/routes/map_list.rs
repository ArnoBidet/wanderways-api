use rocket::serde::json::Json;

use crate::bll::map_list::map_list;
use crate::bo::map::Map;

use super::guards::language_guard::Language;

// @TODO deal with error 500
#[get("/map/list")]
pub async fn get_map_list(language: Language) -> Json<Vec<Map>> {
    map_list(String::from(language.0)).await.unwrap()
}