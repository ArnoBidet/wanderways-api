use crate::{bo::map::Map, dal::map_list::map_list as dal_map_list, PgDatabase};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

pub async fn map_list(
    client: &Connection<PgDatabase>,
    language: String,
    limit: Option<i32>,
    offset: Option<i32>,
    with_tags: Option<Vec<&str>>,
    order_by: Option<&str>,
    direction: Option<&str>,
) -> Result<Json<Vec<Map>>, Error> {
    dal_map_list(client, language, limit, offset, with_tags, order_by, direction)
        .await
        .and_then(|res| Ok(Json(res)))
}
