use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{bo::tag::TagGroup, dal::tag_list::tag_list as dal_tag_list};

pub async fn tag_list(language : String) -> Result<Json<Vec<TagGroup>>, Error> {
    dal_tag_list(language).await.and_then(|res| Ok(Json(res)))
}
