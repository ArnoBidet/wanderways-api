
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::ops::Add;
#[path = "utils"]
mod utils{
    pub mod schemas;
    pub mod sql;
}


fn main() {
    let contents = extract_content_from("data");

    let deserialized: Vec<utils::schemas::Data> = serde_yaml::from_str(&contents).unwrap();

    let mut file = File::create("assets/insert.sql").unwrap();
    file.write_all(insert_data(deserialized).as_bytes()).unwrap();
}

fn extract_content_from(file_name : &str) -> String{
    fs::read_to_string(format!("./assets/{}.yaml",file_name)).expect("Should have been able to read the file")
}


fn insert_languages(languages: Vec<utils::schemas::Language>) -> String {
    let mut languages_value_lines: Vec<String> = Vec::new();
    languages.iter().for_each(|ele|{
        languages_value_lines.push(utils::sql::gen_value_line(vec![
            &ele.id,
        ]));
    }); 
    let languages_inserts = utils::sql::gen_insert("languages", "(id)", languages_value_lines);
    return languages_inserts;
}


fn insert_data(data: Vec<utils::schemas::Data>) -> String {
	let mut data_value_lines: Vec<String> = Vec::new();
	let mut translations_value_lines: Vec<String> = Vec::new();
    data.iter().for_each(|ele|{
        data_value_lines.push(utils::sql::gen_value_line(vec![
            &ele.id,
            &utils::sql::string_or_null(&ele.flag_url),
            &utils::sql::string_or_null(&ele.group),
            &utils::sql::string_or_null(&ele.capital),
            &utils::sql::string_or_null(&ele.numeric_code)
        ]));       
        translations_value_lines.append(&mut gen_translations_values(&ele.name, &ele.id));
    }); 
    let data_translations = utils::sql::gen_insert("translations", "(id_lang, flag_url, data_group, data_capital, numeric_code)", translations_value_lines);
    let data_inserts = utils::sql::gen_insert("data", "(id, flag_url, data_group, data_capital, numeric_code)", data_value_lines);
    return data_translations.add(&data_inserts);
}

fn gen_translations_values(translations : &utils::schemas::Translations, id_item : &String)-> Vec<String>{
    let mut value_lines = Vec::new();
    let mut translations_map = HashMap::new();
    translations_map.insert("en-US", &translations.en_us);
    translations_map.insert("fr-FR", &translations.fr_fr);
    translations_map.insert("de-DE", &translations.de_de);
    translations_map.insert("es-ES", &translations.es_es);
    translations_map.insert("pt-PT", &translations.pt_pt);

    translations_map.keys().for_each(|key|{
        if let Some(translations) = &translations_map.get(key).unwrap() {
            let mut value_line = Vec::new();
            for translation in translations {
                value_line.push(utils::sql::gen_value_line(vec![key,translation,id_item]))
            }
            value_lines.append(&mut value_line);
        }
    });

    value_lines
}
