use crate::bll::map_list::map_list;
use crate::bo::map::Map;

use super::guards::language_guard::Language;
use super::responders::translated_response::TranslatedResponse;

// @TODO deal with error 500
#[get("/map/list")]
pub async fn get_map_list(language: Language) -> TranslatedResponse<Vec<Map>> {
    let json_response = map_list(String::from(&language.0)).await.unwrap();
    TranslatedResponse{
        body:json_response,
        language: language.0
    }
}