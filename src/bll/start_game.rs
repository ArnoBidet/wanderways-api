use rocket_db_pools::Connection;
use tokio_postgres::Error;

use crate::{
    bo::session_data::SessionGeoData, dal::start_game::start_game as dal_start_game, PgDatabase,
};

pub async fn start_game(
    client: &Connection<PgDatabase>,
    id_lang: &str,
    id_map: &str,
) -> Result<Vec<SessionGeoData>, Error> {
    dal_start_game(client, id_lang, id_map)
        .await
        .and_then(|res| Ok(res))
}
