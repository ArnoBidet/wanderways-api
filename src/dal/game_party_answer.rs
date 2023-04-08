use tokio_postgres::{Error, Row};

use crate::dal::query::query;

pub async fn game_party_answer() -> Result<Vec<String>, Error> {

    let sql_query = "SELECT translated_value as answer FROM translation;";
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