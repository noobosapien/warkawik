use serde_yaml;
use std::collections::HashMap;
use std::env;

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    #[cfg(not(test))]
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();

        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();

        Config { map }
    }

    #[cfg(test)]
    pub fn new() -> Config {
        let mut map: HashMap<String, serde_yaml::Value> = HashMap::new();

        map.insert(
            String::from("OPEN_AI_KEY"),
            serde_yaml::from_str("ABC").unwrap(),
        );
        map.insert(
            String::from("OPEN_AI_ORG"),
            serde_yaml::from_str("DEF").unwrap(),
        );

        Config { map }
    }
}
