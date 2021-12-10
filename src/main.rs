use std::io::Write;

// (char, color)
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

struct Game {
    board: [[char; 3]; 3],
    current_turn: Player,
    winner: Player,
}

impl Game {
    fn init(&mut self) {
        self.current_turn = Player::Cross;
        self.board = [[' '; 3]; 3];
    }

    fn is_game_over(&mut self) -> bool {
        for i in 1..3 {
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
                self.winner = match self.board[i][0] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Circle,
                };

                println!("Returning true for 2");

                return true;
            }
        }

        return false;
    }

    fn print_board(&self) {
        for row in self.board {
            println!("-------------");

            for (index, col) in row.iter().enumerate() {
                let color = get_color(*col);

                let string = if index == 0 {
                    format!("| {}{}{} ", color, col, RESET)
                } else if index == 1 {
                    format!("| {}{}{} |", color, col, RESET)
                } else {
                    format!(" {}{}{} |", color, col, RESET)
                };

                print!("{}", &string);
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
        println!("-------------");
    }
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

fn show_error(error_name: Errors, user_input: &str, character: char) {
    match error_name {
        Errors::OverwritingCell => {
            print!(
                "{}Cell #{} already occupied by {}{}\n",
                RED, user_input, character, RESET
            );
        }

        Errors::InputNumberTooLarge => {
            println!("{} Cell number should be between 1 and 9 {}", RED, RESET);
        }

        Errors::IndexOutOfBounds => {
            println!("{} Index out of bounds {}", RED, RESET);
        }
    }
}

fn main() {
    let mut game: Game = Game {
        board: [[' '; 3]; 3],
        current_turn: Player::Circle,
        winner: Player::Circle,
    };

    loop {
        game.print_board();

        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let cell_number: usize = user_input
            .trim()
            .parse()
            .expect("Expected a positive integer");

        if cell_number < 1 || cell_number > 9 {
            show_error(Errors::InputNumberTooLarge, &user_input, ' ');
            continue;
        }

        let (row, col) = get_row_col_value(cell_number - 1);

        if row > 2 || col > 2 {
            show_error(Errors::IndexOutOfBounds, &user_input, ' ');
            continue;
        }

        if game.board[row][col] != ' ' {
            show_error(Errors::OverwritingCell, &user_input, game.board[row][col]);
            continue;
        }

        match game.current_turn {
            Player::Circle => game.board[row][col] = 'O',
            Player::Cross => game.board[row][col] = 'X',
        }

        if game.is_game_over() {
            game.print_board();
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
