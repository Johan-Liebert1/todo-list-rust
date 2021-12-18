extern crate serde;
extern crate serde_json;

// #[macro_use]
extern crate serde_derive;

use ncurses as nc;
use std::fs;
use std::path::Path;

mod colors;
mod constants;
mod helpers;
mod types;

fn main() {
    let file_path = Path::new("data/data.json");
    let file_data = fs::read_to_string(&file_path).expect("could not open file");

    let parsed_json: types::Json = serde_json::from_str(&file_data).unwrap();

    nc::initscr();

    nc::start_color();
    nc::init_pair(constants::NOT_COMPLETED, nc::COLOR_WHITE, nc::COLOR_BLACK);
    nc::init_pair(
        constants::NOT_COMPLETED_HIGHLIGHT,
        nc::COLOR_BLACK,
        nc::COLOR_WHITE,
    );
    nc::init_pair(constants::COMPLETED, nc::COLOR_GREEN, nc::COLOR_BLACK);
    nc::init_pair(
        constants::COMPLETED_HIGHLIGHT,
        nc::COLOR_BLACK,
        nc::COLOR_GREEN,
    );

    let mut quit = false;
    let mut current_selected: usize = 0;

    while !quit {
        for (index, todo) in parsed_json.todoList.iter().enumerate() {
            // helpers::print_item(index + 1, 0 + 1, todo);
            nc::mv(index as i32, 0);

            let attribute = helpers::get_text_attribute(todo, index, current_selected);

            nc::attron(nc::COLOR_PAIR(attribute));
            nc::addstr(&todo.title);
            nc::attroff(nc::COLOR_PAIR(attribute));
        }
        nc::refresh();

        let key: i32 = nc::getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                if current_selected > 0 {
                    current_selected -= 1;
                }
            }
            's' => current_selected += 1,
            _ => {}
        }
    }
}
