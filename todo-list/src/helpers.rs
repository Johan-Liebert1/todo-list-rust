use crate::constants;
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
