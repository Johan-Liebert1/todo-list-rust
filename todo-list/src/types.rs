extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

use crate::actions::UserActions;
use crate::constants;
use crate::layout;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArgJson {
    pub title: String,
    pub desc: Vec<String>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ListType {
    Projects,
    Todo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub index: usize,
    pub title: String,
    pub completed: bool,
    pub description: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub todoList: Vec<Todo>,
    pub projectsList: Vec<Todo>,
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
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
        let usize_width = layout.width as usize;

        // the title
        let symbol = if self.completed { "+" } else { " " };
        let string1 = format!("{}. [{}] {}", index, symbol, self.title);

        let chars_to_fill = usize_width - string1.chars().count();

        vector.push(string1 + &characters_to_fill(chars_to_fill));
        vector.push(characters_to_fill(usize_width)); // extra line between title and description

        for (index, desc) in self.description.iter().enumerate() {
            let total_description_length = desc.chars().count();

            let desc_index = String::from((index + 65) as u8 as char) + &String::from(". ");
            let desc_index_chars = desc_index.chars().count();

            if total_description_length + 3 > usize_width {
                // break the string into segments and add them to vector
                for i in 0..((total_description_length / usize_width) + 1) {
                    let starting_space = if i == 0 {
                        desc_index_chars
                    } else {
                        desc_index_chars * 2
                    };

                    let cut_from = if i == 0 {
                        0
                    } else {
                        i * usize_width - desc_index_chars
                    };

                    let cut_to = min(
                        usize_width * (i + 1) - desc_index_chars,
                        total_description_length,
                    );

                    let s = String::from(&desc[cut_from..cut_to]);
                    let len_cut_string = s.chars().count();

                    vector.push(
                        characters_to_fill(starting_space)
                            + if i == 0 { &desc_index } else { "" }
                            + &s
                            + &characters_to_fill(max(usize_width - len_cut_string, 0)),
                    );
                }
            } else {
                let mut string2 = format!("{}{}", desc_index, desc);
                let chars_to_fill = usize_width - string2.chars().count();

                string2 += &characters_to_fill(chars_to_fill);

                vector.push(string2);
            }
        }

        vector
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

impl Json {
    pub fn insert_into_list(&mut self, list_type: ListType, item: Todo) {
        let list = match list_type {
            ListType::Todo => &mut self.todoList,
            ListType::Projects => &mut self.projectsList,
        };

        let index = if list.len() == 0 || item.index >= list.len() {
            list.len()
        } else {
            item.index
        };

        list.insert(index, item);
    }

    pub fn delete_from_list(
        &mut self,
        list_type: &ListType,
        current_selected: usize,
        user_actions: &mut UserActions,
    ) {
        let list = match list_type {
            ListType::Todo => &mut self.todoList,
            ListType::Projects => &mut self.projectsList,
        };

        // list = list.iter().filter(|item| item.index == to_delete).collect();
        let item_removed = list.remove(current_selected);

        user_actions.push(constants::DELETE_ITEM, *list_type, item_removed);
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
