use rocket_db_pools::Connection;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

use crate::PgDatabase;

pub async fn query(
    sql_query: &str,
    params: &[&(dyn ToSql + Sync)],
    client: &Connection<PgDatabase>
) -> Result<Vec<Row>, Error> {
    let statement = client.prepare(sql_query).await.unwrap(); // can't fail
    let sql_result = client.query(&statement, params).await;
    sql_result
}
