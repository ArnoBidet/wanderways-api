use tokio_postgres::{Error, Row};
use crate::bo::map::Map;
use crate::translation_parser::TranslationParser;

use super::establish_connection::establish_connection;

pub async fn map_list(lang : String) -> Result<Vec<Map>, Error> {
    let (client, handle) = establish_connection().await.unwrap();
    println!("SELECT id_map, map_label, tags, map_description, url_wiki, play_count FROM map_list('{}');",lang);
    let rows = match client.query(&format!("SELECT id_map, map_label, tags, map_description, url_wiki, play_count FROM map_list('{}');", lang), &[]).await {
        Ok(rows) => rows,
        Err(err) => return Err(err),
    };
    handle.abort();
    Ok(rows_to_map(rows))
}

fn rows_to_map(rows: Vec<Row>) -> Vec<Map> {
    let mut result: Vec<Map> = vec![];

    for row in rows {
        result.push(Map {
            id_map: row.get("id_map"),
            map_label :row.get::<&str,String>("map_label").translation_parser(),
            tags :row.get::<&str,String>("tags").translation_parser(),
            map_description :row.get::<&str,String>("map_description").translation_parser(),
            url_wiki :row.get("url_wiki"),
            play_count :row.get("play_count"),
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
