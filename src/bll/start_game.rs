use rocket_sync_db_pools::postgres::Client;
use tokio_postgres::Error;

use crate::{bo::session_data::SessionGeoData, dal::start_game::start_game as dal_start_game};

pub fn start_game(
    id_lang: &str,
    id_map: &str,
    client: &mut Client,
) -> Result<Vec<SessionGeoData>, Error> {
    dal_start_game(id_lang, id_map, client).and_then(|res| Ok(res))
}
