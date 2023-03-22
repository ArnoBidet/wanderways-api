use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub name: Translations,
    pub id: String,
    pub group: Option<String>,
    pub numeric_code: Option<String>,
    pub capital: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Utils {
    pub name: Translations,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translations {
    #[serde(alias = "fr-FR")]
    pub fr_fr: Option<Vec<String>>,
    #[serde(alias = "en-US")]
    pub en_us: Option<Vec<String>>,
    #[serde(alias = "de-DE")]
    pub de_de: Option<Vec<String>>,
    #[serde(alias = "es-ES")]
    pub es_es: Option<Vec<String>>,
    #[serde(alias = "pt-PT")]
    pub pt_pt: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapData {
    pub id_map: String,
    pub id_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub name: Translations,
    pub id: String,
    pub description_translations: Translations,
    pub id_description: String,
    pub url_wiki: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamemodMap {
    pub id_map: String,
    pub id_gamemod: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TagMap {
    pub id_map: String,
    pub id_tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gamemod {
    pub name: Translations,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: Translations,
    pub id: String,
    pub id_group: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagGroup {
    pub name: Translations,
    pub id: String,
}
