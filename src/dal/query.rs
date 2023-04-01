use tokio_postgres::{Error, Row};
use super::establish_connection::establish_connection;
use tokio_postgres::types::ToSql;

pub async fn query(sql_query : &str, params : &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>{
    let (client, handle) = establish_connection().await.unwrap();
    let statement = client.prepare(sql_query).await.unwrap(); // can't fail
    let sql_result = client.query(&statement, params).await;
    handle.abort(); // closes connection after query
    sql_result
}