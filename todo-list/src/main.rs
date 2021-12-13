use serde_json::Value;
use std::fs;
use std::path::Path;

mod constants;

fn main() {
    let file_path = Path::new("data.json");

    let file_data = fs::read_to_string(&file_path).expect("could not open file");

    let v: Value = serde_json::from_str(&file_data).unwrap();

    let pretty = format!("{:#}", v);

    println!("{}", pretty);

    println!("{}", v[constants::TODO_LIST]);
}
