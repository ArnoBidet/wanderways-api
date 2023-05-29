use crate::{
    bo::average_awareness::AverageAwareness,
    dal::average_awareness::average_awareness as dal_average_awareness,
};
use rocket::serde::json::Json;
use rocket_sync_db_pools::postgres::{Client,Error};

pub fn average_awareness(
    id_map: &str,
    id_gamemod: &str,
    id_lang: &Option<String>,
    client: &mut Client,
) -> Result<Json<AverageAwareness>, Error> {
    dal_average_awareness(id_map, id_gamemod, id_lang, client).and_then(|res| Ok(Json(res)))
}
