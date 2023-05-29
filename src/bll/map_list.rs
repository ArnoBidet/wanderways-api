use crate::{bo::map::Map, dal::map_list::map_list as dal_map_list};
use rocket::serde::json::Json;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::Error;

pub fn map_list(language: String, client: &mut Client) -> Result<Json<Vec<Map>>, Error> {
    dal_map_list(language, client).and_then(|res| Ok(Json(res)))
}
