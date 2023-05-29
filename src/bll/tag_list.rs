use rocket::serde::json::Json;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::Error;

use crate::{bo::tag::TagGroup, dal::tag_list::tag_list as dal_tag_list};

pub fn tag_list(language: String, client: &mut Client) -> Result<Json<Vec<TagGroup>>, Error> {
    dal_tag_list(language, client).and_then(|res| Ok(Json(res)))
}
