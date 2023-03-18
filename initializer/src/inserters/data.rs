use std::ops::Add;
use crate::utils::*;
use crate::inserters::translation::*;

pub fn gen_data() -> String {
    let data = yaml_reader::yaml_reader::<Vec<schemas::Data>>("data");
    let mut data_value_lines: Vec<String> = Vec::new();
    let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele| {
        sql::gen_value_line(
            vec![
                &ele.id,
                &sql::string_or_null(&ele.group),
                &sql::string_or_null(&ele.capital),
                &sql::string_or_null(&ele.numeric_code),
            ],
            &mut data_value_lines,
        );
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
    });
    let data_translations = sql::gen_insert(
        "translations",
        "(id_lang, data_group, data_capital, numeric_code)",
        &translations_value_lines,
    );
    let data_inserts = gen_translation_insert(&data_value_lines);
    return data_translations.add(&data_inserts);
}