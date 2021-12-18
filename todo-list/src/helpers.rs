use ncurses as nc;

use crate::constants;
use crate::layout;
use crate::types;

pub fn get_text_attribute(todo_item: &types::Todo, index: usize, current_selected: i16) -> i16 {
    return if todo_item.completed {
        if index as i16 == current_selected {
            constants::COMPLETED_HIGHLIGHT
        } else {
            constants::COMPLETED
        }
    } else {
        if index as i16 == current_selected {
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
    current_selected: i16,
    current_tab: &types::ListType,
) {
    let starting_x = layout.position.x;
    nc::mv(0, starting_x as i32);

    nc::clrtoeol(); // clear the first line as characters from previous render were showing up

    nc::attron(nc::COLOR_PAIR(constants::TAB_COLOR));
    nc::addstr(list_type.title(current_tab));
    nc::attroff(nc::COLOR_PAIR(constants::TAB_COLOR));

    let mut row = 2;

    for (index, todo) in list.iter().enumerate() {
        let mut string_vector: Vec<String> = Vec::new();

        nc::mv(row, starting_x);

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
