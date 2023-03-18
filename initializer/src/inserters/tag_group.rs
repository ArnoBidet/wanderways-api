use std::ops::Add;
use crate::utils::*;

use super::translation::*;

pub fn gen_tag_group() -> String {
    let data = yaml_reader::yaml_reader::<Vec<schemas::TagGroup>>("tag_group");
    let mut tag_group_value_lines: Vec<String> = Vec::new();
    let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele| {
        sql::gen_value_line(
            vec![&ele.id],
            &mut tag_group_value_lines,
        );
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
    });
    let tag_group_translations = gen_translation_insert(&translations_value_lines);
    let tag_group_inserts = sql::gen_insert(
        "tag_group",
        "(id)",
        &tag_group_value_lines
    );
    return tag_group_translations.add(&tag_group_inserts);
}