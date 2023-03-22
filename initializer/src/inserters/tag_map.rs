use crate::utils::*;

pub fn gen_tag_map() -> String {
    let tag_map = yaml_reader::yaml_reader::<Vec<schemas::TagMap>>("tag_map");
    let mut tag_map_value_lines: Vec<String> = Vec::new();
    tag_map.iter().for_each(|ele| {
        sql::gen_value_line(vec![&ele.id_map, &ele.id_tag], &mut tag_map_value_lines);
    });
    let tag_map_inserts =
        sql::gen_insert("tag_map", "(id_map, id_tag)", &tag_map_value_lines);
    return tag_map_inserts;
}
