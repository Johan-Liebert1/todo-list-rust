use ncurses as nc;

use crate::constants;
use crate::layout;
use crate::types;

pub fn get_text_attribute(todo_item: &types::Todo, index: usize, current_selected: usize) -> i16 {
    return if todo_item.completed {
        if index == current_selected {
            constants::COMPLETED_HIGHLIGHT
        } else {
            constants::COMPLETED
        }
    } else {
        if index == current_selected {
            constants::NOT_COMPLETED_HIGHLIGHT
        } else {
            constants::NOT_COMPLETED
        }
    };
}

pub fn render_list(
    list_type: types::ListType,
    list: &Vec<types::Todo>,
    layout: &layout::LayoutBox,
    current_selected: usize,
) {
    let starting_x = layout.position.x;
    nc::mv(0, starting_x as i32);

    nc::attron(nc::COLOR_PAIR(constants::TITLE_COLOR));
    nc::addstr(list_type.title());
    nc::attroff(nc::COLOR_PAIR(constants::TITLE_COLOR));

    for (index, todo) in list.iter().enumerate() {
        nc::mv((index + 2) as i32, starting_x as i32);

        let attribute = get_text_attribute(todo, index, current_selected);

        nc::attron(nc::COLOR_PAIR(attribute));
        nc::addstr(&todo.to_string(index + 1));
        nc::attroff(nc::COLOR_PAIR(attribute));
    }

    nc::refresh();
}
