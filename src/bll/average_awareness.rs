use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{
    bo::average_awareness::AverageAwareness,
    dal::average_awareness::average_awareness as dal_average_awareness,
};

pub async fn average_awareness(
    id_map: &str,
    id_gamemod: &str,
    id_lang: &Option<String>,
) -> Result<Json<AverageAwareness>, Error> {
    dal_average_awareness(id_map, id_gamemod, id_lang)
        .await
        .and_then(|res| Ok(Json(res)))
}
