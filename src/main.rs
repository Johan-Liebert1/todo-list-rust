use std::io::Write;

// (char, color)
#[derive(Debug)]
enum Player {
    Cross,
    Circle,
}

fn get_color(character: char) -> String {
    return match character {
        'X' => String::from("\u{001b}[31;1m"),
        'O' => String::from("\u{001b}[32;1m"),
        _ => String::from(""),
    };
}

struct Game {
    board: [[char; 3]; 3],
    game_over: bool,
    current_turn: Player,
    winner: Player,
}

impl Game {
    fn init(&mut self) {
        self.game_over = false;
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
        let reset = "\u{001b}[0m";

        for row in self.board {
            println!("-------------");

            for (index, col) in row.iter().enumerate() {
                let color = get_color(*col);

                let string = if index == 0 {
                    format!("| {}{}{} ", color, col, reset)
                } else if index == 1 {
                    format!("| {}{}{} |", color, col, reset)
                } else {
                    format!(" {}{}{} |", color, col, reset)
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

fn main() {
    let mut game: Game = Game {
        board: [[' '; 3]; 3],
        current_turn: Player::Circle,
        winner: Player::Circle,
        game_over: false,
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

        let (row, col) = get_row_col_value(cell_number);

        match game.current_turn {
            Player::Circle => game.board[row][col] = 'O',
            Player::Cross => game.board[row][col] = 'X',
        }

        if game.is_game_over() {
            game.print_board();
            println!("{:?} Won!", game.winner);
            break;
        }

        // change turn
        game.current_turn = match game.current_turn {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        }
    }
}
