use std::{error::Error, fs};

use serde_yaml::Mapping;

pub fn parse_yaml(path: &str) -> Result<Mapping, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let yml = serde_yaml::from_str::<serde_yaml::Value>(&contents)?;
    let only_map: Result<Mapping, Box<dyn Error>> = match yml {
        serde_yaml::Value::Mapping(map) => Ok(map),
        _ => Err("Invalid YAML file".into()),
    };
    only_map
}
