use crate::bo::map::Map;
use crate::translation_parser::TranslationParser;
use crate::PgDatabase;
use rocket_db_pools::Connection;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub async fn map_list(
    client: &Connection<PgDatabase>,
    lang: String,
    limit: Option<i32>,
    offset: Option<i32>,
    with_tags: Option<Vec<&str>>,
    order_by: Option<&str>,
    direction: Option<&str>,
) -> Result<Vec<Map>, Error> {
    let mut params : Vec<&(dyn ToSql + Sync+ 'static)> = Vec::new();
    let mut current_index = 1;
    let mut sql_query = format!("SELECT id, label, tags, description, url_wiki, play_count FROM f_map_list(${})",current_index);
    params.push(&lang);

    let tags = with_tags.clone().unwrap_or_default().iter().map(|value|format!("(?=.*{})", value)).collect::<Vec<String>>().join("");
    params.push(&tags);
    println!("{}",&tags);
    match with_tags {
        Some(_) => {
            current_index+=1;
            sql_query.push_str(format!(" WHERE tags ~* ${}",current_index).as_str());
        },
        None=> {
            params.pop();
        }
    }

    params.push(&limit);
    match limit {
        Some(_) => {
            current_index+=1;
            println!("{}",format!(" LIMIT ${}",current_index).as_str());
            sql_query.push_str(format!(" LIMIT ${}::INT",current_index).as_str());
        },
        None=> {
            params.pop();
        }
    }

    params.push(&offset);
    match offset {
        Some(_) => {
            current_index+=1;
            sql_query.push_str(format!(" OFFSET ${}::INT",current_index).as_str());
        },
        None=> {
            params.pop();
        }
    }

    match order_by {
        Some(column)=>{
            let ordered_by = match column {
                "id"=> Some("id"),
                "label"=> Some("label"),
                "play_count"=> Some("play_count"),
                _ => None
            };

            let direction = match direction {
                Some(dir)=>{
                    match dir{
                        "desc" => "DESC",
                        "DESC" => "DESC",
                        _ => "ASC"
                    }
                }
                _ => "ASC"
            };
            if ordered_by.is_some(){
                sql_query.push_str(format!(" ORDER BY {} {}",ordered_by.unwrap(), direction).as_str());
            }
        },
        None => {}
    }    

    sql_query.push_str(";");
    let statement = client.prepare(sql_query.as_str()).await.unwrap();
    println!("{} {:?}",sql_query, statement.params());
    match client.query(&statement, &params).await {
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
