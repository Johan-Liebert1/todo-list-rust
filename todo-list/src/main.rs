extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actions::UserActions;
use ncurses as nc;
use std::path::Path;
use std::{env, fs};

mod actions;
mod constants;
mod helpers;
mod layout;
mod types;

use types::Json;

fn move_cursor_up(current_selected: &mut i16, list_len: usize) {
    if *current_selected > 0 {
        *current_selected -= 1;
    } else {
        *current_selected = (list_len - 1) as i16;
    }
}

fn move_cursor_down(current_selected: &mut i16, list_len: usize) {
    if *current_selected < (list_len - 1) as i16 {
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
    nc::init_pair(constants::TAB_COLOR, nc::COLOR_CYAN, nc::COLOR_BLACK);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{:?}", args);
        return;
    }

    let file_path = Path::new(constants::FILE_PATH);
    let file_data = fs::read_to_string(&file_path).expect("could not open file");

    let mut parsed_json: Json = serde_json::from_str(&file_data).unwrap();
    let mut user_actions: UserActions = UserActions {
        user_actions: Vec::new(),
    };

    init_ncurses();

    let left_layout = layout::LayoutBox {
        width: nc::COLS() / 2 - constants::PADDING,
        height: nc::LINES(),
        layout_type: layout::LayoutTypes::Vertical,
        position: layout::Position { x: 0, y: 0 },
    };

    let right_layout = layout::LayoutBox {
        width: nc::COLS() / 2 - constants::PADDING,
        height: nc::LINES(),
        layout_type: layout::LayoutTypes::Vertical,
        position: layout::Position {
            x: nc::COLS() / 2,
            y: 0,
        },
    };

    let mut quit = false;
    let mut current_selected_todo: i16 = 0;
    let mut current_selected_project: i16 = -1;
    let mut current_tab: types::ListType = types::ListType::Todo;

    while !quit {
        helpers::render_list(
            types::ListType::Todo,
            &parsed_json.todoList,
            &left_layout,
            current_selected_todo,
            &current_tab,
        );

        helpers::render_list(
            types::ListType::Projects,
            &parsed_json.projectsList,
            &right_layout,
            current_selected_project,
            &current_tab,
        );
        nc::refresh();

        let key: i32 = nc::getch();

        // println!("key = {}", key);

        match key as u8 as char {
            'w' => match current_tab {
                types::ListType::Todo => {
                    move_cursor_up(&mut current_selected_todo, parsed_json.todoList.len())
                }

                types::ListType::Projects => move_cursor_up(
                    &mut current_selected_project,
                    parsed_json.projectsList.len(),
                ),
            },

            's' => match current_tab {
                types::ListType::Todo => {
                    move_cursor_down(&mut current_selected_todo, parsed_json.todoList.len())
                }

                types::ListType::Projects => move_cursor_down(
                    &mut current_selected_project,
                    parsed_json.projectsList.len(),
                ),
            },

            '\n' => match current_tab {
                types::ListType::Todo => {
                    parsed_json.todoList[current_selected_todo as usize].toggle_completed()
                }
                types::ListType::Projects => {
                    parsed_json.projectsList[current_selected_project as usize].toggle_completed()
                }
            },

            '\t' | 'a' | 'd' => {
                (current_tab) = match current_tab {
                    types::ListType::Todo => {
                        current_selected_project = 0;
                        current_selected_todo = -1;
                        types::ListType::Projects
                    }

                    types::ListType::Projects => {
                        current_selected_project = -1;
                        current_selected_todo = 0;
                        types::ListType::Todo
                    }
                }
            }

            'D' => {
                parsed_json.delete_from_list(
                    &current_tab,
                    match current_tab {
                        types::ListType::Todo => current_selected_todo as usize,
                        types::ListType::Projects => current_selected_project as usize,
                    },
                    &mut user_actions,
                );
                nc::clear();
            }

            'Z' => {
                user_actions.pop(&mut parsed_json);
                nc::clear();
            }

            'q' => quit = true,

            'S' => {
                quit = true;
                save_and_exit(&parsed_json);
            }
            _ => {}
        }
    }

    nc::endwin();
}
