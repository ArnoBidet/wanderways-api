use crate::{bo::map::Map, dal::map_list::map_list as dal_map_list, PgDatabase};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

pub async fn map_list(language: String, client: &Connection<PgDatabase>) -> Result<Json<Vec<Map>>, Error> {
    dal_map_list(language, client).await.and_then(|res| Ok(Json(res)))
}
