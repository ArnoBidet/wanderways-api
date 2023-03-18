use std::ops::Add;
use crate::utils::*;
use super::translation::*;

pub fn gen_map() -> String {
    let data = yaml_reader::yaml_reader::<Vec<schemas::Map>>("map");
    let mut map_value_lines: Vec<String> = Vec::new();
    let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele| {
        sql::gen_value_line(
            vec![&ele.id, &ele.id_description, &ele.url_wiki],
            &mut map_value_lines,
        );
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
        translations_value_lines.append(&mut gen_translations_values(
            &ele.description_translations,
            &ele.id_description,
        ));
    });
    let map_translations = gen_translation_insert(&translations_value_lines);
    let map_inserts = sql::gen_insert(
        "map",
        "(id, id_description, url_wiki)",
        &map_value_lines
    );
    return map_translations.add(&map_inserts);
}