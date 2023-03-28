use rocket::serde::json::Json;

use crate::bll::map_list::map_list;
use crate::bo::map::Map;

// @TODO deal with error 500
#[get("/map/list")]
pub async fn get_map_list() -> Json<Vec<Map>> {
    map_list().await.unwrap()
}
