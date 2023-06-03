use crate::PgDatabase;
use crate::bo::average_awareness::{AverageAwareness, AverageAwarenessItem};
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};
use rocket_db_pools::Connection;
use crate::dal::query::query;

pub async fn average_awareness(
    id_map: &str,
    id_gamemod: &str,
    id_lang: &Option<String>,
    client: &Connection<PgDatabase>,
) -> Result<AverageAwareness, Error> {
    let sql_query_average_awareness = "SELECT id, found_count FROM f_average_awareness($1,$2,$3);";

    let sql_query_play_count = "SELECT play_count FROM f_play_count($1,$2,$3);";
    let params: &[&(dyn ToSql + Sync)] = &[&id_map, &id_gamemod, &id_lang];
    println!("{}", sql_query_average_awareness);
    let average_awareness_items = match query(sql_query_average_awareness, &params, client).await {
        Ok(rows) => rows_to_average_awareness_item(rows),
        Err(err) => return Err(err),
    };
    match query(sql_query_play_count, &params, client).await {
        Ok(rows) => Ok(rows_to_average_awareness(rows, average_awareness_items)),
        Err(err) => Err(err),
    }
}

fn rows_to_average_awareness_item(rows: Vec<Row>) -> Vec<AverageAwarenessItem> {
    let mut result: Vec<AverageAwarenessItem> = vec![];

    for row in rows {
        result.push(AverageAwarenessItem {
            id: row.get("id"),
            found_count: row.get("found_count"),
        });
    }
    result
}

fn rows_to_average_awareness(
    rows: Vec<Row>,
    average_awareness_items: Vec<AverageAwarenessItem>,
) -> AverageAwareness {
    let result = AverageAwareness {
        play_count: match rows.first() {
            Some(row) => row.get("play_count"),
            None => 0,
        },
        data: average_awareness_items,
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rows_to_game_test() {
        let result = rows_to_average_awareness_item(vec![]);
        let data_set: Vec<AverageAwarenessItem> = vec![];
        assert!(result.iter().all(|item| data_set.contains(item)));
    }
}
