use std::env;
use tokio::task::JoinHandle;
use tokio_postgres::NoTls;
use tokio_postgres::{Client, Error};

pub async fn establish_connection() -> Result<(Client, JoinHandle<()>), Error> {
    let database_url =
        String::from(env::var("DATABASE_URL").expect(
            "DATABASE_URL environment variable was not found. Please, add it to .env file.",
        ));
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    let handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok((client, handle))
}
