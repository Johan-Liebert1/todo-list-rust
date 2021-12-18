extern crate serde;
extern crate serde_json;

// #[macro_use]
extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod constants;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    completed: bool,
    description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Json {
    todoList: Vec<Todo>,
    projectsList: Vec<Todo>,
}

// impl Display for Todo {

// }

fn main() {
    let file_path = Path::new("data/data.json");
    let file_data = fs::read_to_string(&file_path).expect("could not open file");

    let parsed_json: Json = serde_json::from_str(&file_data).unwrap();

    for (index, todo) in parsed_json.todoList.iter().enumerate() {
        println!("{}. {:?}", index, todo);
    }
}
