use crate::{bo::session_data::SessionGeoData, dal::query::query};
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub fn start_game(
    id_lang: &str,
    id_map: &str,
    client: &mut Client,
) -> Result<Vec<SessionGeoData>, Error> {
    let sql_query = "SELECT id, data_label FROM f_map_geo_data($1, $2)";
    let params: &[&(dyn ToSql + Sync)] = &[&id_lang, &id_map];

    match query(sql_query, &params, client) {
        Ok(rows) => Ok(rows_to_game_answer(rows)),
        Err(err) => Err(err),
    }
}

fn rows_to_game_answer(rows: Vec<Row>) -> Vec<SessionGeoData> {
    let mut result: Vec<SessionGeoData> = vec![];

    for row in &rows {
        result.push(SessionGeoData {
            id: row.get("id"),
            translations: vec![row.get("data_label")],
        });
    }

    result
}
