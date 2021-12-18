extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub todoList: Vec<Todo>,
    pub projectsList: Vec<Todo>,
}

impl Todo {
    pub fn to_string(&self, index: usize) -> String {
        let symbol = if self.completed { "O" } else { " " };
        format!("{}. [{}] {}", index, symbol, self.title)
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}
