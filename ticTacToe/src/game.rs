use crate::colors;
use crate::helpers;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum Player {
    Cross,
    Circle,
}

pub struct Game {
    pub board: [[char; 3]; 3],
    pub current_turn: Player,
    pub winner: Player,
    pub is_draw: bool,
}

impl Game {
    pub fn init(&mut self) {
        self.current_turn = Player::Cross;
        self.board = [[' '; 3]; 3];
        self.is_draw = false;
    }

    pub fn print_current_turn(&self) {
        let current_symbol = match self.current_turn {
            Player::Circle => 'O',
            Player::Cross => 'X',
        };

        let symbol_color = helpers::get_color(current_symbol);

        print!(
            "{}Current Turn: {}{}{}{}\nEnter the cell number > {}",
            colors::LIGHT_CYAN,
            symbol_color,
            current_symbol,
            colors::RESET,
            colors::LIGHT_CYAN,
            colors::RESET
        );

        std::io::stdout().flush().unwrap();
    }

    pub fn is_game_over(&mut self) -> bool {
        let mut is_board_filled = true;

        'outer: for row in self.board {
            for col in row {
                if col == ' ' {
                    is_board_filled = false;
                    break 'outer;
                }
            }
        }

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
                self.winner = match self.board[0][2] {
                    'X' => Player::Cross,
                    'O' => Player::Circle,
                    _ => Player::Circle,
                };

                return true;
            }
        }

        if is_board_filled {
            self.is_draw = true;
            return true;
        }

        return false;
    }

    pub fn print_board(&self) {
        println!();
        for (row_idx, row) in self.board.iter().enumerate() {
            if row_idx != 0 {
                println!("-------------");
            }

            for (index, col) in row.iter().enumerate() {
                let color = helpers::get_color(*col);

                let string = if index == 0 {
                    format!("  {}{}{} ", color, col, colors::RESET)
                } else if index == 1 {
                    format!("| {}{}{} |", color, col, colors::RESET)
                } else {
                    format!(" {}{}{}  ", color, col, colors::RESET)
                };

                print!("{}", &string);
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
        println!();
    }
}
