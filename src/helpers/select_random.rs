use std::fs;
use serde_json;

pub fn select_random_problem() {
    let path = "./data.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data)
        .expect("Unable to parse");
    println!("{res}");
}
