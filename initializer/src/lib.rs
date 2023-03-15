use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Translations {
    #[serde(alias = "fr-FR")]
    fr_FR: Option<Vec<String>>,
    #[serde(alias = "en-US")]
    en_US: Option<Vec<String>>,
    #[serde(alias = "de-De")]
    de_DE: Option<Vec<String>>,
    #[serde(alias = "es-ES")]
    es_ES: Option<Vec<String>>,
    #[serde(alias = "pt-PT")]
    pt_PT: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: Translations,
    id: String,
    flag_url: Option<String>,
    group: Option<String>,
    numeric_code: Option<String>,
    capital: Option<String>,
}

pub fn load() {
    let contents =
        fs::read_to_string("./assets/data.yaml").expect("Should have been able to read the file");

    let deserialized: Vec<Data> = serde_yaml::from_str(&contents).unwrap();

    let mut file = File::create("assets/insert.sql").unwrap();
    file.write_all(generate_data_insert(deserialized).as_bytes()).unwrap();
}

fn generate_data_insert(data: Vec<Data>) -> String {
    let mut data_value: Vec<String> = Vec::new();
    let mut data_translations: Vec<String> = Vec::new();
    for ele in data {
        data_value.push(format!(
            "\n(\"{}\", {}, {}, {}, {})",
            ele.id,
            string_or_null(&ele.flag_url),
            string_or_null(&ele.group),
            string_or_null(&ele.capital),
            string_or_null(&ele.numeric_code)
        ));
        data_translations.push(generate_translations_insert(&ele.name, &ele.id))
    }
    let result = format!("{}{}",data_translations.join(""),generate_insert("data", "(id, flag_url, data_group, data_capital, numeric_code)", data_value));
    return result;
}

fn generate_translations_insert(translations : &Translations, id_item : &String)-> String{
    let mut result = Vec::new();
    if let Some(french_translations) = &translations.en_US {
        result.append(&mut generate_translation_values("en-US", french_translations, id_item));
    }
    if let Some(french_translations) = &translations.fr_FR {
        result.append(&mut generate_translation_values("fr-FR", french_translations, id_item));
    }
    if let Some(french_translations) = &translations.de_DE {
        result.append(&mut generate_translation_values("de-DE", french_translations, id_item));
    }
    if let Some(french_translations) = &translations.es_ES {
        result.append(&mut generate_translation_values("es-ES", french_translations, id_item));
    }
    if let Some(french_translations) = &translations.pt_PT {
        result.append(&mut generate_translation_values("pt-PT", french_translations, id_item));
    }
    return generate_insert("translations", "(id_lang, flag_url, data_group, data_capital, numeric_code)", result);
}

fn generate_translation_values(language : &str, translations : &Vec<String>, id_item : &String)-> Vec<String>{
    let mut result = Vec::new();
    for translation in translations {
        result.push(format!("({},{},{})",language,translation,id_item,))
    }
    return result;
}

fn generate_insert(table_name : &str, schema : &str, values_list : Vec<String>) -> String{
    format!("INSERT INTO {} {} VALUES {};\n", table_name, schema, values_list.join(","))
}

fn string_or_null(value: &Option<String>) -> String {
    return match value {
        Some(val) => format!("\"{}\"", val),
        None => String::from("null"),
    };
}

