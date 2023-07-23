use rocket_db_pools::Connection;

use crate::{bll::map_geo_data::map_geo_data, bo::geo_data::GeoData, PgDatabase};

use super::{
    guards::language_guard::Language, responders::translated_response::TranslatedResponse,
};

// @TODO deal with error 500
#[get("/map-geo-data/<map_id>")]
pub async fn get_map_geo_data(
    map_id: String,
    language: Language,
    client: Connection<PgDatabase>,
) -> TranslatedResponse<Vec<GeoData>> {
    let request_language = language.0.clone();
    let json_response = map_geo_data(&client, request_language, map_id)
        .await
        .unwrap();
    TranslatedResponse {
        body: json_response,
        language: language.0,
    }
}
