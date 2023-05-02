use std::env;
use tokio::task::JoinHandle;
use tokio_postgres::{Client, Error};
use tokio_postgres::{Config, NoTls};

pub async fn establish_connection() -> Result<(Client, JoinHandle<()>), Error> {

    let (client, connection) = Config::new()
        .host(&env::var("DB_HOST").unwrap())
        .user(&env::var("DB_USER").unwrap())
        .port(env::var("DB_PORT").unwrap().parse::<u16>().unwrap())
        .password(env::var("DB_PASS").unwrap())
        .dbname(&env::var("DB_NAME").unwrap())
        .connect(NoTls)
        .await
        .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    let handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok((client, handle))
}
