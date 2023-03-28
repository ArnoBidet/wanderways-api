use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{bo::map::Map, dal::map_list::map_list as dal_map_list};

pub async fn map_list() -> Result<Json<Vec<Map>>, Error> {
    dal_map_list(String::from("fr-FR")).await.and_then(|res| Ok(Json(res)))
}
