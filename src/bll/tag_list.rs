use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

use crate::{bo::tag::TagGroup, dal::tag_list::tag_list as dal_tag_list, PgDatabase};

pub async fn tag_list(language: String, client: &Connection<PgDatabase>) -> Result<Json<Vec<TagGroup>>, Error> {
    dal_tag_list(language, client).await.and_then(|res| Ok(Json(res)))
}
