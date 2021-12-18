extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

use crate::layout;

#[derive(PartialEq)]
pub enum ListType {
    Projects,
    Todo,
}

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

fn min(a: usize, b: usize) -> usize {
    return if a < b { a } else { b };
}

fn characters_to_fill(num_chars: usize) -> String {
    let mut s = String::new();

    for _ in 0..num_chars {
        s += " ";
    }

    s
}

impl Todo {
    pub fn to_string<'a>(
        &self,
        index: usize,
        layout: &layout::LayoutBox,
        vector: &'a mut Vec<String>,
    ) -> &'a mut Vec<String> {
        let symbol = if self.completed { "+" } else { " " };
        let mut string1 = format!("{}. [{}] {}", index, symbol, self.title);

        let chars_to_fill = layout.width - string1.chars().count() as i32;

        for _ in 0..chars_to_fill {
            string1 += " ";
        }

        vector.push(string1);

        let total_description_length = self.description.chars().count();
        let usize_width = layout.width as usize;

        if total_description_length + 3 > usize_width {
            // break the string into segments and add them to vector
            for i in 0..((total_description_length / usize_width) + 1) {
                let cut_from = if i == 0 { 0 } else { i * usize_width - 3 };
                let cut_to = min(usize_width * (i + 1) - 3, total_description_length);

                let s = String::from(&self.description[cut_from..cut_to]);

                vector.push(String::from("   ") + &s + &characters_to_fill(s.chars().count() - 3));
            }

            return vector;
        }

        let mut string2 = format!("   {}", self.description);
        let chars_to_fill = usize_width - string2.chars().count();

        string2 += &characters_to_fill(chars_to_fill);

        vector.push(string2);

        return vector;
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

impl ListType {
    pub fn title(&self, current_tab: &ListType) -> &str {
        match (self, current_tab) {
            (ListType::Projects, ListType::Projects) => "[Projects]",
            (ListType::Projects, ListType::Todo) => "Projects",
            (ListType::Todo, ListType::Todo) => "[Todo]",
            (ListType::Todo, ListType::Projects) => "Todo",
        }
    }
}
