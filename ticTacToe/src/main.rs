use std::io::Write;

pub mod colors;
pub mod game;
pub mod helpers;
pub mod minimax;

fn main() {
    let mut game: game::Game = game::Game {
        board: [[' '; 3]; 3],
        current_turn: game::Player::Circle,
        winner: game::Player::Circle,
    };

    let mut errors_in_buffer: Vec<String> = Vec::new();

    loop {
        helpers::clear_screen();

        game.print_board();

        if errors_in_buffer.len() > 0 {
            let error = errors_in_buffer.pop().unwrap();
            println!("{}", error);
        }

        let mut user_input = String::new();

        game.print_current_turn();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let cell_number: usize = user_input
            .trim()
            .parse()
            .expect("Expected a positive integer");

        if cell_number < 1 || cell_number > 9 {
            helpers::show_error(
                helpers::Errors::InputNumberTooLarge,
                &user_input,
                ' ',
                &mut errors_in_buffer,
            );
            continue;
        }

        let (row, col) = helpers::get_row_col_value(cell_number - 1);

        if row > 2 || col > 2 {
            helpers::show_error(
                helpers::Errors::IndexOutOfBounds,
                &user_input,
                ' ',
                &mut errors_in_buffer,
            );
            continue;
        }

        if game.board[row][col] != ' ' {
            helpers::show_error(
                helpers::Errors::OverwritingCell,
                &user_input,
                game.board[row][col],
                &mut errors_in_buffer,
            );
            continue;
        }

        match game.current_turn {
            game::Player::Circle => game.board[row][col] = 'O',
            game::Player::Cross => game.board[row][col] = 'X',
        }

        if game.is_game_over() {
            helpers::clear_screen();
            game.print_board();
            println!("{}{:?} Won!{}", colors::GREEN, game.winner, colors::RESET);

            let mut play_again = String::new();

            print!("\nPlay again? [yes/no] ");
            std::io::stdout().flush().unwrap();

            std::io::stdin()
                .read_line(&mut play_again)
                .expect("Couldn't read input");

            let start_new_game = match play_again.to_lowercase().trim() {
                "y" | "yes" => true,
                _ => false,
            };

            if !start_new_game {
                break;
            }

            game.init();
            continue;
        }

        // change turn
        game.current_turn = match game.current_turn {
            game::Player::Circle => game::Player::Cross,
            game::Player::Cross => game::Player::Circle,
        }
    }
}
