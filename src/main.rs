use std::{fmt::format, io::Write};

enum Player {
    Cross = 0,
    Circle = 1,
}

struct Game {
    // board: Vec<Vec<char>>,
    board: [[char; 3]; 3],
    is_game_over: bool,
    current_turn: Player,
    winner: Player,
}

impl Game {
    fn init(&mut self) {
        self.is_game_over = false;
        self.current_turn = Player::Cross;
        self.board = [[' '; 3]; 3];
    }

    fn print_board(self) {
        for row in self.board {
            println!("-------------");

            for (index, col) in row.iter().enumerate() {
                let string = if index == 0 {
                    format!("| {} ", col)
                } else if index == 1 {
                    format!("| {} |", col)
                } else {
                    format!(" {} |", col)
                };

                print!("{}", &string);
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
        println!("-------------");
    }
}

fn main() {
    let mut game: Game = Game {
        board: [[' '; 3]; 3],
        current_turn: Player::Circle,
        winner: Player::Circle,
        is_game_over: false,
    };

    game.print_board();
}
