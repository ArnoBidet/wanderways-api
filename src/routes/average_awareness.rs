use crate::{
    bll::average_awareness::average_awareness, bo::average_awareness::AverageAwareness, PgDatabase,
};

use super::{responders::translated_response::TranslatedResponse, utils::lang_utils::get_lang};

// @TODO deal with error 500
#[get("/average-awareness/<id_map>/<id_gamemod>?<id_lang>")]
pub async fn get_average_awareness(
    id_map: String,
    id_gamemod: String,
    id_lang: Option<String>,
    conn: PgDatabase,
) -> TranslatedResponse<AverageAwareness> {
    let lang: Option<String> = match id_lang {
        Some(lang) => Some(get_lang(lang)), // if provided but not recognized then fallback to en-US
        None => None,
    };
    let clone_lang = lang.clone(); // makes me want to cry
    let json_response = conn
        .run(move |client| {
            average_awareness(id_map.as_str(), id_gamemod.as_str(), &clone_lang, client)
        })
        .await
        .unwrap();

    TranslatedResponse {
        body: json_response,
        language: lang.unwrap_or(String::from("")), // empty content language means language agnostic statistics
    }
}
