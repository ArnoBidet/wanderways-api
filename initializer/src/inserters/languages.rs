use crate::{extract_content_deserialized, utils};

pub fn gen_languages() -> String {
    let languages = extract_content_deserialized::<Vec<utils::schemas::Language>>("languages");
    let mut languages_value_lines: Vec<String> = Vec::new();
    languages.iter().for_each(|ele| {
        utils::sql::gen_value_line(vec![&ele.id], &mut languages_value_lines);
    });
    let languages_inserts = utils::sql::gen_insert("languages", "(id)", &languages_value_lines);
    return languages_inserts;
}