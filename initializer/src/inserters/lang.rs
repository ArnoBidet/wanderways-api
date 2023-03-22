use crate::utils::*;

pub fn gen_lang() -> String {
    let langs = yaml_reader::yaml_reader::<Vec<schemas::Lang>>("lang");
    let mut langs_value_lines: Vec<String> = Vec::new();
    langs.iter().for_each(|ele| {
        sql::gen_value_line(vec![&ele.id], &mut langs_value_lines);
    });
    let langs_inserts = sql::gen_insert("lang", "(id)", &langs_value_lines);
    return langs_inserts;
}