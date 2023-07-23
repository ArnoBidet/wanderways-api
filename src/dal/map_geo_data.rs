use crate::PgDatabase;
use crate::bo::geo_data::{GeoData, GeoDataItem};
use crate::dal::query::query;
use crate::translation_parser::TranslationParser;
use rocket_db_pools::Connection;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub async fn map_geo_data(
    client: &Connection<PgDatabase>,
    lang: String,
    map_id: String,
) -> Result<Vec<GeoData>, Error> {
    let sql_query =
        "SELECT id,data_label,id_group,group_label,id_capital,capital_label,numeric_code
        FROM f_map_geo_data($1, $2);";
    let params: &[&(dyn ToSql + Sync)] = &[&lang, &map_id];
    match query(client, sql_query, params).await {
        Ok(rows) => Ok(rows_to_geo_data(rows)),
        Err(err) => Err(err),
    }
}

fn rows_to_geo_data(rows: Vec<Row>) -> Vec<GeoData> {
    let mut result: Vec<GeoData> = vec![];

    for row in rows {
        let group = match row.get::<&str, Option<String>>("id_group") {
            Some(_) => Some(GeoDataItem {
                id: row.get("id_group"),
                label: row.get::<&str, String>("group_label").translation_parser(),
            }),
            None => None,
        };
        let capital = match row.get::<&str, Option<String>>("id_capital") {
            Some(_) => Some(GeoDataItem {
                id: row.get("id_capital"),
                label: row
                    .get::<&str, String>("capital_label")
                    .translation_parser(),
            }),
            None => None,
        };
        result.push(GeoData {
            id: row.get("id"),
            label: row.get::<&str, String>("data_label").translation_parser(),
            group,
            capital,
            numeric_code: row.get("numeric_code"),
        });
    }
    result
}
