use rocket::serde::json::Json;
use tokio_postgres::Error;

use crate::{bo::geo_data::GeoData, dal::map_geo_data::map_geo_data as dal_map_geo_data};

pub async fn map_geo_data(language : String, id_map : String) -> Result<Json<Vec<GeoData>>, Error> {
    dal_map_geo_data(language, id_map).await.and_then(|res| Ok(Json(res)))
}
