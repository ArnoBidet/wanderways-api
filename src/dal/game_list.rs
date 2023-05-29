use crate::bo::game::Game;
use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::{Error, Row};

use crate::dal::query::query;

pub fn game_list(client: &mut Client) -> Result<Vec<Game>, Error> {
    let sql_query = "SELECT id, play_count FROM v_game_list;";
    let params = &[];
    match query(sql_query, params, client) {
        Ok(rows) => Ok(rows_to_game(rows)),
        Err(err) => Err(err),
    }
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
