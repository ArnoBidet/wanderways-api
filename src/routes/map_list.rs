use super::guards::language_guard::Language;
use super::responders::translated_response::TranslatedResponse;
use crate::bll::map_list::map_list;
use crate::bo::map::Map;
use crate::PgDatabase;
use rocket_db_pools::Connection;

/// |Parameter|Type|Description|
/// |-|-|-|
/// |limit | number | Limits the number of results returned. e.g `limit=5`|
/// |offset | number | The number of items to skip before starting to collect the result set (allows pagination systems)|
/// |with_tags | string[] | A list of tags that the maps must match. e.g `with_tags=bar&with_tags=foo`|
/// |order_by | string | By what attribute the values should be ordered by. Supports `id`, `label` and `play_count`. e.g `order_by=play_count`|
/// |direction | number | Order of result (default normal, desc means reverse). Needs parameter `order_by` to work.|
#[get("/map/list?<limit>&<offset>&<with_tags>&<order_by>&<direction>")]
pub async fn get_map_list(
    language: Language,
    client: Connection<PgDatabase>,
    limit: Option<i32>,
    offset: Option<i32>,
    with_tags: Option<Vec<&str>>,
    order_by: Option<&str>,
    direction: Option<&str>,
) -> TranslatedResponse<Vec<Map>> {
    let request_language = language.0.clone();
    let json_response = map_list(&client, request_language, limit, offset, with_tags, order_by, direction)
        .await
        .unwrap();
    TranslatedResponse {
        body: json_response,
        language: language.0,
    }
}
