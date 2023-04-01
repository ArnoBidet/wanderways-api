use crate::bo::map::Map;
use crate::translation_parser::TranslationParser;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

use crate::dal::query::query;

pub async fn map_list(lang: String) -> Result<Vec<Map>, Error> {
    let sql_query = "SELECT id_map, map_label, tags, map_description, url_wiki, play_count FROM map_list($1);";
    let params : &[&(dyn ToSql + Sync)] = &[&lang];
    match query(sql_query,params).await {
        Ok(rows)=> Ok(rows_to_map(rows)),
        Err(err)=> Err(err)
    }
}

fn rows_to_map(rows: Vec<Row>) -> Vec<Map> {
    let mut result: Vec<Map> = vec![];

    for row in rows {
        result.push(Map {
            id_map: row.get("id_map"),
            map_label: row.get::<&str, String>("map_label").translation_parser(),
            tags: row.get::<&str, String>("tags").translation_parser(),
            map_description: row
                .get::<&str, String>("map_description")
                .translation_parser(),
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
