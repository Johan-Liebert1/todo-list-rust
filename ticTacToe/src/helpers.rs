use crate::colors;
use crate::game;
use std::io::Write;
pub enum Errors {
    InputNumberTooLarge,
    IndexOutOfBounds,
    OverwritingCell,
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn equal3(a: char, b: char, c: char) -> bool {
    a != ' ' && a == b && b == c
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

pub fn handle_game_over(game: &game::Game) -> bool {
    clear_screen();
    game.print_board();

    if game.is_draw {
        println!("{}It's a tie!{}", colors::CYAN, colors::RESET);
    } else {
        println!("{}{:?} Won!{}", colors::GREEN, game.winner, colors::RESET);
    }

    let mut play_again = String::new();

    print!("\nPlay again? [yes/no] ");
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut play_again)
        .expect("Couldn't read input");

    return match play_again.to_lowercase().trim() {
        "y" | "yes" => true,
        _ => false,
    };
}

pub fn check_for_errors(
    game: &game::Game,
    cell_number: usize,
    user_input: &str,
    errors_in_buffer: &mut Vec<String>,
) -> (i8, i8) {
    if cell_number < 1 || cell_number > 9 {
        show_error(
            Errors::InputNumberTooLarge,
            &user_input,
            ' ',
            errors_in_buffer,
        );
        return (-1, -1);
    }

    let (row, col) = get_row_col_value(cell_number - 1);

    if row > 2 || col > 2 {
        show_error(Errors::IndexOutOfBounds, &user_input, ' ', errors_in_buffer);
        return (-1, -1);
    }

    if game.board[row][col] != ' ' {
        show_error(
            Errors::OverwritingCell,
            &user_input,
            game.board[row][col],
            errors_in_buffer,
        );
        return (-1, -1);
    }

    return (row as i8, col as i8);
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
