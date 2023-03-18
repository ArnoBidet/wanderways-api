/// module definition
#[path = "utils"]
mod utils {
    pub mod schemas;
    pub mod sql;
}
#[path = "inserters"]
mod inserters {
    pub mod data;
    pub mod gamemod_map;
    pub mod gamemod;
    pub mod languages;
    pub mod map_data;
    pub mod map;
    pub mod tag_group;
    pub mod tag_map;
    pub mod tag;
    pub mod translation;
}

use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use inserters::*;

fn main() {
    let to_add = &mut vec![];
    to_add.push(data::gen_data());
    to_add.push(languages::gen_languages());
    to_add.push(gamemod_map::gen_gamemod_map());
    to_add.push(map_data::gen_map_data());
    to_add.push(map::gen_map());
    to_add.push(gamemod::gen_gamemod());
    to_add.push(tag_group::gen_tag_group());
    to_add.push(tag::gen_tag());
    to_add.push(tag_map::gen_tag_map());
    
    let mut result_file = File::create("assets/insert.sql").unwrap();
    for inserts in to_add {
        result_file.write_all(inserts.as_bytes()).unwrap();
    }
}

fn extract_content_deserialized<T: DeserializeOwned>(file_name: &str) -> T {
    let file = fs::File::open(&format!("./assets/{}.yaml", file_name))
        .expect("Should have been able to read the file");
    serde_yaml::from_reader(file).expect(&format!("Could not read values for file {}.", file_name)) // type inference
}