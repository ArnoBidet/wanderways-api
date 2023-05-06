use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

use crate::{dal::query::query, bo::session_data::SessionGeoData};

pub async fn start_game(id_lang : &str, id_map : &str) -> Result<Vec<SessionGeoData>, Error> {

    let sql_query = "SELECT id, data_label FROM f_map_geo_data($1, $2)";
    let params: &[&(dyn ToSql + Sync)]  = &[&id_lang, &id_map];

    match query(sql_query, &params).await {
        Ok(rows)=> Ok(rows_to_game_answer(rows)),
        Err(err)=> Err(err)
    }
}

fn rows_to_game_answer(rows: Vec<Row>) -> Vec<SessionGeoData> {

    let mut result: Vec<SessionGeoData> = vec![];

    for row in &rows {
        result.push(SessionGeoData {
            id: row.get("id"),
            translations: vec![row.get("data_label")]
        });
    }

    result
}