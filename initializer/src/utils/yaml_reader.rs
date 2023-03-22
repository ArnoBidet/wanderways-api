use serde::de::DeserializeOwned;
use std::fs;

pub fn yaml_reader<T: DeserializeOwned>(file_name: &str) -> T {
    let file = fs::File::open(&format!("./assets/{}.yaml", file_name))
        .expect("Should have been able to read the file");
    serde_yaml::from_reader(file).expect(&format!("Could not read values for file {}.", file_name)) // type inference
}