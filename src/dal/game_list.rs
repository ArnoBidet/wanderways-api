use tokio_postgres::{Error, Row};

use crate::bo::game::Game;

use super::establish_connection::establish_connection;

pub async fn game_list() -> Result<Vec<Game>, Error> {
    let (client, handle) = match establish_connection().await {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    
    let rows = match client.query("SELECT * FROM game_list;", &[]).await {
        Ok(rows) => rows,
        Err(err) => return Err(err),
    };
    handle.abort();
    Ok(rows_to_game(rows))
}

fn rows_to_game(rows: Vec<Row>) -> Vec<Game> {
    let mut result: Vec<Game> = vec![];

    for row in rows {
        result.push(Game {
            id: row.get("id"),
            play_count: row.get("play_count"),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rows_to_game_test() {
        let result = rows_to_game(vec![]);
        let data_set: Vec<Game> = vec![];
        assert!(result.iter().all(|item| data_set.contains(item)));
    }
}
