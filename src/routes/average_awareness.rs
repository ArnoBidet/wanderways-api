use crate::{bo::average_awareness::AverageAwareness, bll::average_awareness::average_awareness};

use super::{utils::lang_utils::get_lang, responders::translated_response::TranslatedResponse};

// @TODO deal with error 500
#[get("/average-awareness/<id_map>/<id_gamemod>?<id_lang>")]
pub async fn get_average_awareness(
    id_map: &str,
    id_gamemod: &str,
    id_lang: Option<&str>,
) -> TranslatedResponse<AverageAwareness> {
    let lang :Option<String>= match id_lang {
        Some(lang)=> Some(get_lang(lang)), // if provided but not recognized then fallback to en-US
        None => None
    };
    let json_response = average_awareness(id_map, id_gamemod, &lang)
        .await
        .unwrap();
    TranslatedResponse{
        body: json_response,
        language: lang.unwrap_or(String::from("")) // empty content language means language agnostic statistics
    }
}
