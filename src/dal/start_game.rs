use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};
use std::collections::HashMap;

use crate::{dal::query::query, bo::session_data::SessionGeoData};

pub async fn start_game(id_lang : &str, id_map : &str) -> Result<HashMap<std::string::String, SessionGeoData>, Error> {

    let sql_query = "SELECT id_item, translated_value FROM f_game_data($1)";
    let params: &[&(dyn ToSql + Sync)]  = &[&id_lang];
    //let params: &[&(dyn ToSql + Sync)]  = &[&id_lang, &id_map];


    match query(sql_query, &params).await {
        Ok(rows)=> Ok(rows_to_game_answer(rows)),
        Err(err)=> Err(err)
    }
}

fn rows_to_game_answer(rows: Vec<Row>) -> HashMap<std::string::String, SessionGeoData> {

    let mut result: HashMap<String, SessionGeoData> = HashMap::new();

    for row in &rows {
        let id: String = row.get("id_item");
        let translation: String = row.get("translated_value");
        let translation_vec = vec![translation];

        let session_geo_data = SessionGeoData {
            id,
            translations: translation_vec,
        };

        // Check whether an object with the same id already exists in the HashMap
        if let Some(existing_data) = result.get_mut(&session_geo_data.id) {
            existing_data.translations.extend(session_geo_data.translations);
        } else {
            result.insert(session_geo_data.id.clone(), session_geo_data);
        }
    }

    dbg!(&result);
    std::process::exit(1);
    result
}