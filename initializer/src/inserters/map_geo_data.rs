use crate::utils::*;

pub fn gen_map_geo_data() -> String {
    let map_geo_data = yaml_reader::yaml_reader::<Vec<schemas::MapData>>("map_geo_data");
    let mut map_geo_data_value_lines: Vec<String> = Vec::new();
    map_geo_data.iter().for_each(|ele| {
        sql::gen_value_line(vec![&ele.id_map, &ele.id_geo_data], &mut map_geo_data_value_lines);
    });
    let map_geo_data_inserts = sql::gen_insert("map_geo_data", "(id_map, id_geo_data)", &map_geo_data_value_lines);
    return map_geo_data_inserts;
}