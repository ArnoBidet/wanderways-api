use crate::bll::tag_list::tag_list;
use crate::bo::tag::TagGroup;

use super::guards::language_guard::Language;
use super::responders::translated_response::TranslatedResponse;

// @TODO deal with error 500
#[get("/tag/list")]
pub async fn get_tag_list(language: Language) -> TranslatedResponse<Vec<TagGroup>> {
    let json_response = tag_list(String::from(&language.0)).await.unwrap();
    TranslatedResponse{
        body:json_response,
        language: language.0
    }
}
