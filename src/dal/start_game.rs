use tokio_postgres::{Error, Row};

use crate::dal::query::query;

pub async fn start_game(id_lang : String, id_map : String) -> Result<Vec<String>, Error> {

    let sql_query = "SELECT id, data_label FROM f_map_geo_data('fr-FR', 'FRANCE_DEPARTMENTS');";
    let params  = &[];

    match query(sql_query,params).await {
        Ok(rows)=> Ok(rows_to_game_answer(rows)),
        Err(err)=> Err(err)
    }
}

fn rows_to_game_answer(rows: Vec<Row>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for row in rows {
        result.push(row.get("answer"));
    }
    result
}