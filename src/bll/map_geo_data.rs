use crate::{bo::geo_data::GeoData, dal::map_geo_data::map_geo_data as dal_map_geo_data};
use rocket::serde::json::Json;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::Error;

pub fn map_geo_data(
    language: String,
    id_map: String,
    client: &mut Client,
) -> Result<Json<Vec<GeoData>>, Error> {
    dal_map_geo_data(language, id_map, client).and_then(|res| Ok(Json(res)))
}
