extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use ncurses as nc;
use std::fs;
use std::path::Path;

mod colors;
mod constants;
mod helpers;
mod layout;
mod types;

use types::Json;

fn move_cursor_up(current_selected: &mut usize, list_len: usize) {
    if *current_selected > 0 {
        *current_selected -= 1;
    } else {
        *current_selected = list_len - 1;
    }
}

fn move_cursor_down(current_selected: &mut usize, list_len: usize) {
    if *current_selected < list_len - 1 {
        *current_selected += 1
    } else {
        *current_selected = 0
    }
}

fn save_and_exit(parsed_json: &Json) {
    let deserialised_json = serde_json::to_string(parsed_json).unwrap();

    let file_path = Path::new(constants::FILE_PATH);
    fs::write(file_path, deserialised_json).expect("Failed to write to file");
}

fn init_ncurses() {
    nc::initscr();
    nc::noecho(); // don't show typed characters on the terminal
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // hide the cursor

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
    nc::init_pair(constants::TITLE_COLOR, nc::COLOR_CYAN, nc::COLOR_BLACK);
}

fn main() {
    let file_path = Path::new(constants::FILE_PATH);
    let file_data = fs::read_to_string(&file_path).expect("could not open file");

    let mut parsed_json: Json = serde_json::from_str(&file_data).unwrap();

    init_ncurses();

    let root_layout = layout::LayoutBox {
        width: nc::COLS(),
        height: nc::LINES(),
        layout_type: layout::LayoutTypes::Horizontal,
        children: Vec::new(),
        position: layout::Position { x: 0, y: 0 },
    };

    let left_layout = layout::LayoutBox {
        width: nc::COLS() / 2,
        height: nc::LINES(),
        layout_type: layout::LayoutTypes::Vertical,
        children: Vec::new(),
        position: layout::Position { x: 0, y: 0 },
    };

    let right_layout = layout::LayoutBox {
        width: nc::COLS() / 2,
        height: nc::LINES(),
        layout_type: layout::LayoutTypes::Vertical,
        children: Vec::new(),
        position: layout::Position {
            x: nc::COLS() / 2,
            y: 0,
        },
    };

    let mut quit = false;
    let mut current_selected: usize = 0;

    while !quit {
        helpers::render_list(
            types::ListType::Todo,
            &parsed_json.todoList,
            &left_layout,
            current_selected,
        );

        helpers::render_list(
            types::ListType::Projects,
            &parsed_json.projectsList,
            &right_layout,
            current_selected,
        );

        let key: i32 = nc::getch();

        // println!("key = {}", key);

        match key {
            27 => move_cursor_up(&mut current_selected, parsed_json.todoList.len()),
            29 => move_cursor_down(&mut current_selected, parsed_json.todoList.len()),
            10 => parsed_json.todoList[current_selected].toggle_completed(),
            _ => {}
        }

        match key as u8 as char {
            'q' => quit = true,
            's' => {
                quit = true;
                save_and_exit(&parsed_json);
            }
            _ => {}
        }
    }

    nc::endwin();
}
