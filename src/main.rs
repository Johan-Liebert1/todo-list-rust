use std::io::Write;

#[derive(Debug)]
enum Player {
    Cross,
    Circle,
}

enum Errors {
    InputNumberTooLarge,
    IndexOutOfBounds,
    OverwritingCell,
}

const RED: &str = "\u{001b}[31;1m";
const GREEN: &str = "\u{001b}[32;1m";
const WHITE: &str = "\u{001b}[37;1m";
const BLUE: &str = "\u{001b}[34;1m";
const RESET: &str = "\u{001b}[0m";
const LIGHT_CYAN: &str = "\u{001b}[36m";

struct Game {
    board: [[char; 3]; 3],
    current_turn: Player,
    winner: Player,
}

impl Game {
    #[allow(unused)]
    fn init(&mut self) {
        self.current_turn = Player::Cross;
        self.board = [[' '; 3]; 3];
    }

    fn print_current_turn(&self) {
        let current_symbol = match self.current_turn {
            Player::Circle => 'O',
            Player::Cross => 'X',
        };

        let symbol_color = get_color(current_symbol);

        print!(
            "{}Current Turn: {}{}{}{}\nEnter the cell number > {}",
            LIGHT_CYAN, symbol_color, current_symbol, RESET, LIGHT_CYAN, RESET
        );

        std::io::stdout().flush().unwrap();
    }

    fn is_game_over(&mut self) -> bool {
        for i in 0..3 {
            if self.board[i][0] != ' '
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
            {
                self.winner = match self.board[i][0] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Cross,
                };

                return true;
            }

            if self.board[0][i] != ' '
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
            {
                self.winner = match self.board[0][i] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Circle,
                };

                return true;
            }

            if self.board[0][0] != ' '
                && self.board[0][0] == self.board[1][1]
                && self.board[1][1] == self.board[2][2]
            {
                self.winner = match self.board[0][0] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Circle,
                };

                return true;
            }

            if self.board[0][2] != ' '
                && self.board[0][2] == self.board[1][1]
                && self.board[1][1] == self.board[2][0]
            {
                self.winner = match self.board[0][0] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Circle,
                };

                return true;
            }
        }

        return false;
    }

    fn print_board(&self) {
        for (row_idx, row) in self.board.iter().enumerate() {
            if row_idx != 0 {
                println!("-------------");
            }

            for (index, col) in row.iter().enumerate() {
                let color = get_color(*col);

                let string = if index == 0 {
                    format!("  {}{}{} ", color, col, RESET)
                } else if index == 1 {
                    format!("| {}{}{} |", color, col, RESET)
                } else {
                    format!(" {}{}{}  ", color, col, RESET)
                };

                print!("{}", &string);
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_row_col_value(number: usize) -> (usize, usize) {
    let row = number / 3;
    let col = number % 3;

    return (row, col);
}

fn get_color(character: char) -> &'static str {
    return match character {
        'X' => BLUE,
        'O' => GREEN,
        _ => WHITE,
    };
}

fn show_error(
    error_name: Errors,
    user_input: &str,
    character: char,
    errors_in_buffer: &mut Vec<String>,
) {
    let error_string = match error_name {
        Errors::OverwritingCell => format!(
            "{}Cell #{} already occupied by {}{}\n",
            RED,
            user_input.trim(),
            character,
            RESET
        ),

        Errors::InputNumberTooLarge => {
            format!("{}Cell number should be between 1 and 9 {}", RED, RESET)
        }

        Errors::IndexOutOfBounds => format!("{}Index out of bounds {}", RED, RESET),
    };

    errors_in_buffer.push(error_string);
}

fn main() {
    let mut game: Game = Game {
        board: [[' '; 3]; 3],
        current_turn: Player::Circle,
        winner: Player::Circle,
    };

    let mut errors_in_buffer: Vec<String> = Vec::new();

    loop {
        clear_screen();

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
            show_error(
                Errors::InputNumberTooLarge,
                &user_input,
                ' ',
                &mut errors_in_buffer,
            );
            continue;
        }

        let (row, col) = get_row_col_value(cell_number - 1);

        if row > 2 || col > 2 {
            show_error(
                Errors::IndexOutOfBounds,
                &user_input,
                ' ',
                &mut errors_in_buffer,
            );
            continue;
        }

        if game.board[row][col] != ' ' {
            show_error(
                Errors::OverwritingCell,
                &user_input,
                game.board[row][col],
                &mut errors_in_buffer,
            );
            continue;
        }

        match game.current_turn {
            Player::Circle => game.board[row][col] = 'O',
            Player::Cross => game.board[row][col] = 'X',
        }

        if game.is_game_over() {
            println!("{}{:?} Won!{}", GREEN, game.winner, RESET);
            break;
        }

        // change turn
        game.current_turn = match game.current_turn {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        }
    }
}
