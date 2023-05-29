use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub fn query(
    sql_query: &str,
    params: &[&(dyn ToSql + Sync)],
    client: &mut Client,
) -> Result<Vec<Row>, Error> {
    let statement = client.prepare(sql_query).unwrap(); // can't fail
    let sql_result = client.query(&statement, params);
    sql_result
}
