use std::collections::HashMap;

use crate::utils;

pub fn gen_translation_insert(translations_value_lines : &Vec<String>)-> String {
    utils::sql::gen_insert(
        "translations",
        "(id_lang, map_group, map_capital, numeric_code)",
        &translations_value_lines,
    )
}

pub fn gen_translations_values(
    translations: &utils::schemas::Translations,
    id_item: &String,
) -> Vec<String> {
    let mut value_lines = Vec::new();
    let mut translations_map = HashMap::new();
    translations_map.insert("en-US", &translations.en_us);
    translations_map.insert("fr-FR", &translations.fr_fr);
    translations_map.insert("de-DE", &translations.de_de);
    translations_map.insert("es-ES", &translations.es_es);
    translations_map.insert("pt-PT", &translations.pt_pt);

    translations_map.keys().for_each(|key| {
        if let Some(translations) = &translations_map.get(key).unwrap() {
            let mut value_line = Vec::new();
            for translation in translations {
                utils::sql::gen_value_line(vec![key, translation, id_item], &mut value_line);
            }
            value_lines.append(&mut value_line);
        }
    });

    value_lines
}