use crate::colors;

pub enum Errors {
    InputNumberTooLarge,
    IndexOutOfBounds,
    OverwritingCell,
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn get_row_col_value(number: usize) -> (usize, usize) {
    let row = number / 3;
    let col = number % 3;

    return (row, col);
}

pub fn get_color(character: char) -> &'static str {
    return match character {
        'X' => colors::BLUE,
        'O' => colors::GREEN,
        _ => colors::WHITE,
    };
}

pub fn show_error(
    error_name: Errors,
    user_input: &str,
    character: char,
    errors_in_buffer: &mut Vec<String>,
) {
    let error_string = match error_name {
        Errors::OverwritingCell => format!(
            "{}Cell #{} is already occupied by {}{}\n",
            colors::RED,
            user_input.trim(),
            character,
            colors::RESET
        ),

        Errors::InputNumberTooLarge => {
            format!(
                "{}Cell number should be between 1 and 9 {}",
                colors::RED,
                colors::RESET
            )
        }

        Errors::IndexOutOfBounds => format!("{}Index out of bounds {}", colors::RED, colors::RESET),
    };

    errors_in_buffer.push(error_string);
}
