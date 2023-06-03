use crate::bll::tag_list::tag_list;
use crate::bo::tag::TagGroup;
use crate::PgDatabase;
use rocket_db_pools::Connection;
use super::guards::language_guard::Language;
use super::responders::translated_response::TranslatedResponse;

// @TODO deal with error 500
#[get("/tag/list")]
pub async fn get_tag_list(
    language: Language,
    client: Connection<PgDatabase>,
) -> TranslatedResponse<Vec<TagGroup>> {
    let request_language = language.0.clone();
    let json_response = tag_list(request_language, &client)
        .await
        .unwrap();
    TranslatedResponse {
        body: json_response,
        language: language.0,
    }
}
