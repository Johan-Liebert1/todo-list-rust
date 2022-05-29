use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

use ncurses as nc;

use crate::constants;
use crate::layout;
use crate::types;

fn print_usage() {
    println!("Usage: todo-list [OPTIONS]");
    println!("OPTIONS");
    println!("  -a, --add       todo | project");
    println!("  -d, --data      '{{ \"title\": String, \"desc\": []String, \"imp\": bool }}'",);
    println!("  view            View Json data");
}

pub fn get_file_data() -> String {
    let path = get_file_path();
    let file_path = Path::new(&path);
    let file_data = fs::read_to_string(file_path).expect("could not open file");

    file_data
}

pub fn parse_arguments(args: &Vec<String>) -> (types::ListType, types::ArgJson) {
    if args.len() != 5 && args[1] != "view" {
        print_usage();
        exit(1);
    }

    if args[1] == "view" {
        // view json data
        println!("\nViewing: {}\n", get_file_path());
        println!("{}", get_file_data());
        exit(0);
    }

    let mut todo_or_project = types::ListType::Todo;
    let mut json = String::new();

    let mut i = 1;

    while i < args.len() - 1 {
        let flag = &args[i][..];
        let value = &args[i + 1][..];

        match flag {
            "-d" | "--data" => {
                json = String::from(value);
            }

            "-a" | "--add" => {
                todo_or_project = match value {
                    "todo" => types::ListType::Todo,
                    "project" => types::ListType::Projects,
                    _ => {
                        println!("Invalid Usage for {}", flag);
                        print_usage();
                        exit(1);
                    }
                }
            }
            _ => {}
        }

        i += 2;
    }

    let parsed_json = match serde_json::from_str::<types::ArgJson>(&json) {
        Ok(pj) => pj,
        Err(error) => {
            println!("\nError: {}\n", error);
            print_usage();
            println!();

            exit(1);
        }
    };

    (todo_or_project, parsed_json)
}

pub fn get_file_path() -> String {
    let user_name = env::var("USER").unwrap();

    if constants::DEV_ENV {
        String::from("data/data.json")
    } else {
        format!("/home/{}/.todo-data/data.json", user_name)
    }
}

pub fn get_text_attribute(todo_item: &types::Todo, index: usize, current_selected: i16) -> i16 {
    if todo_item.completed {
        if index as i16 == current_selected {
            constants::COMPLETED_HIGHLIGHT
        } else {
            constants::COMPLETED
        }
    } else if index as i16 == current_selected {
        if todo_item.important {
            constants::IMPORTANT_HIGHLIGHT
        } else {
            constants::NOT_COMPLETED_HIGHLIGHT
        }
    } else {
        if todo_item.important {
            constants::IMPORTANT
        } else {
            constants::NOT_COMPLETED
        }
    }
}

pub fn render_list(
    list_type: types::ListType,
    list: &[types::Todo],
    layout: &layout::LayoutBox,
    current_selected: i16,
    current_tab: &types::ListType,
) {
    let starting_x = layout.position.x;
    let starting_y = layout.position.y;

    nc::mv(starting_y, starting_x);

    nc::clrtoeol(); // clear the first line as characters from previous render were showing up

    nc::attron(nc::COLOR_PAIR(constants::TAB_COLOR));
    nc::addstr(list_type.title(current_tab));
    nc::attroff(nc::COLOR_PAIR(constants::TAB_COLOR));

    let mut row = 2;

    for (index, todo) in list.iter().enumerate() {
        let mut string_vector: Vec<String> = Vec::new();

        let attribute = get_text_attribute(todo, index, current_selected);

        nc::attron(nc::COLOR_PAIR(attribute));

        todo.to_string(index + 1, layout, &mut string_vector);

        for string in string_vector {
            nc::mv(row, starting_x);
            nc::addstr(&string);
            row += 1;
        }

        nc::attroff(nc::COLOR_PAIR(attribute));

        row += 1;
    }
}
