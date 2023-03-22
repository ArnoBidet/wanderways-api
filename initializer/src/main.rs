/// module definition
#[path = "utils"]
mod utils {
    pub mod schemas;
    pub mod sql;
    pub mod yaml_reader;
}
#[path = "inserters"]
mod inserters {
    pub mod geo_data;
    pub mod gamemod_map;
    pub mod gamemod;
    pub mod languages;
    pub mod map_geo_data;
    pub mod map;
    pub mod tag_group;
    pub mod tag_map;
    pub mod tag;
    pub mod translation;
}

use std::fs::File;
use std::io::prelude::*;
use inserters::*;

fn main() {
    let to_add = &mut vec![];
    to_add.push(geo_data::gen_geo_data());
    to_add.push(languages::gen_languages());
    to_add.push(gamemod_map::gen_gamemod_map());
    to_add.push(map_geo_data::gen_map_geo_data());
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