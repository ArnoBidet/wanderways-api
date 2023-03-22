use crate::utils::*;

pub fn gen_languages() -> String {
    let languages = yaml_reader::yaml_reader::<Vec<schemas::Language>>("languages");
    let mut languages_value_lines: Vec<String> = Vec::new();
    languages.iter().for_each(|ele| {
        sql::gen_value_line(vec![&ele.id], &mut languages_value_lines);
    });
    let languages_inserts = sql::gen_insert("languages", "(id)", &languages_value_lines);
    return languages_inserts;
}