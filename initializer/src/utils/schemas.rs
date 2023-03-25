use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub name: Translation,
    pub id: String,
    pub group: Option<String>,
    pub numeric_code: Option<String>,
    pub capital: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Utils {
    pub name: Translation,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
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
    pub id_geo_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub name: Translation,
    pub id: String,
    pub description_translated_values: Translation,
    pub id_description: String,
    pub url_wiki: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lang {
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
    pub name: Translation,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: Translation,
    pub id: String,
    pub id_group: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagGroup {
    pub name: Translation,
    pub id: String,
}
