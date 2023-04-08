use crate::{bll::map_geo_data::map_geo_data, bo::geo_data::GeoData};

use super::{
    guards::language_guard::Language, responders::translated_response::TranslatedResponse,
};

// @TODO deal with error 500
#[get("/map-geo-data/<map_id>")]
pub async fn get_map_geo_data(
    map_id: &str,
    language: Language,
) -> TranslatedResponse<Vec<GeoData>> {
    let json_response = map_geo_data(String::from(&language.0), String::from(map_id))
        .await
        .unwrap();
    TranslatedResponse {
        body: json_response,
        language: language.0,
    }
}
