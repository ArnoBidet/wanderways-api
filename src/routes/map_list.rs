use super::guards::language_guard::Language;
use super::responders::translated_response::TranslatedResponse;
use crate::bll::map_list::map_list;
use crate::bo::map::Map;
use crate::PgDatabase;
use rocket_db_pools::Connection;

// @TODO deal with error 500
#[get("/map/list")]
pub async fn get_map_list(
    language: Language,
    client: Connection<PgDatabase>,
) -> TranslatedResponse<Vec<Map>> {
    let request_language = language.0.clone();
    let json_response = map_list(request_language, &client).await.unwrap();
    TranslatedResponse {
        body: json_response,
        language: language.0,
    }
}
