use std::ops::Add;
use crate::{extract_content_deserialized, utils};
use super::translation::*;

pub fn gen_gamemod() -> String {
    let data = extract_content_deserialized::<Vec<utils::schemas::Gamemod>>("gamemod");
    let mut gamemod_value_lines: Vec<String> = Vec::new();
    let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele| {
        utils::sql::gen_value_line(
            vec![&ele.id],
            &mut gamemod_value_lines,
        );
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
    });
    let gamemod_translations = gen_translation_insert(&translations_value_lines);
    let gamemod_inserts = utils::sql::gen_insert(
        "gamemod",
        "(id)",
        &gamemod_value_lines
    );
    return gamemod_translations.add(&gamemod_inserts);
}