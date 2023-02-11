use std::collections::HashMap;

pub const NOT_COMPLETED: i16 = 0;
pub const NOT_COMPLETED_HIGHLIGHT: i16 = 1;
pub const COMPLETED: i16 = 2;
pub const COMPLETED_HIGHLIGHT: i16 = 10;
pub const TAB_COLOR: i16 = 4;
pub const IMPORTANT: i16 = 5;
pub const IMPORTANT_HIGHLIGHT: i16 = 6;
pub const MAGENTA: i16 = 11;
pub const YELLOW: i16 = ncurses::COLOR_YELLOW;

pub const PADDING: i32 = 5;
pub const DEV_ENV: bool = false;

// action names
pub const DELETE_ITEM: &str = "DELETE_ITEM";

pub fn get_hash_map() -> HashMap<&'static str, i16> {
    HashMap::from([("yellow", YELLOW), ("magenta", MAGENTA)])
}
