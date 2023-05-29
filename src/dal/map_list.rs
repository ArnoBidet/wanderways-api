use crate::bo::map::Map;
use crate::dal::query::query;
use crate::translation_parser::TranslationParser;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub fn map_list(lang: String, client: &mut Client) -> Result<Vec<Map>, Error> {
    let sql_query =
        "SELECT id, label, tags, description, url_wiki, play_count FROM f_map_list($1);";
    let params: &[&(dyn ToSql + Sync)] = &[&lang];
    match query(sql_query, params, client) {
        Ok(rows) => Ok(rows_to_map(rows)),
        Err(err) => Err(err),
    }
}

fn rows_to_map(rows: Vec<Row>) -> Vec<Map> {
    let mut result: Vec<Map> = vec![];

    for row in rows {
        result.push(Map {
            id: row.get("id"),
            label: row.get("label"),
            tags: row.get::<&str, String>("tags").translation_parser(),
            description: row.get("description"),
            url_wiki: row.get("url_wiki"),
            play_count: row.get("play_count"),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rows_to_map_test() {
        let result = rows_to_map(vec![]);
        let data_set: Vec<Map> = vec![];
        assert!(result.iter().all(|item| data_set.contains(item)));
    }
}
