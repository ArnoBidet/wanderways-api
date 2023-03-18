use crate::{extract_content_deserialized, utils};

pub fn gen_map_data() -> String {
    let map_data = extract_content_deserialized::<Vec<utils::schemas::MapData>>("map_data");
    let mut map_data_value_lines: Vec<String> = Vec::new();
    map_data.iter().for_each(|ele| {
        utils::sql::gen_value_line(vec![&ele.id_map, &ele.id_data], &mut map_data_value_lines);
    });
    let map_data_inserts = utils::sql::gen_insert("map_data", "(id_map, id_data)", &map_data_value_lines);
    return map_data_inserts;
}