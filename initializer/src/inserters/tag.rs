use std::ops::Add;
use crate::utils::*;
use super::translation::*;

pub fn gen_tag() -> String {
    let data = yaml_reader::yaml_reader::<Vec<schemas::Tag>>("tag");
    let mut tag_value_lines: Vec<String> = Vec::new();
    let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele| {
        sql::gen_value_line(
            vec![&ele.id, &ele.id_group],
            &mut tag_value_lines,
        );
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
    });
    let tag_translations = gen_translation_insert(&translations_value_lines);
    let tag_inserts = sql::gen_insert(
        "tag",
        "(id, id_group)",
        &tag_value_lines
    );
    return tag_translations.add(&tag_inserts);
}