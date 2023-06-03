use crate::{
    bo::average_awareness::AverageAwareness,
    dal::average_awareness::average_awareness as dal_average_awareness, PgDatabase,
};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

pub async fn average_awareness(
    id_map: &str,
    id_gamemod: &str,
    id_lang: &Option<String>,
    client: &Connection<PgDatabase>,
) -> Result<Json<AverageAwareness>, Error> {
    dal_average_awareness(id_map, id_gamemod, id_lang, client).await.and_then(|res| Ok(Json(res)))
}
