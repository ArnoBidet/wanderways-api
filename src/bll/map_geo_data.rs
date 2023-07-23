use crate::{
    bo::geo_data::GeoData, dal::map_geo_data::map_geo_data as dal_map_geo_data, PgDatabase,
};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use tokio_postgres::Error;

pub async fn map_geo_data(
    client: &Connection<PgDatabase>,
    language: String,
    id_map: String,
) -> Result<Json<Vec<GeoData>>, Error> {
    dal_map_geo_data(client, language, id_map)
        .await
        .and_then(|res| Ok(Json(res)))
}
