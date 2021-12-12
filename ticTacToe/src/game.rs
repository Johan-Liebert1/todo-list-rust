use crate::colors;
use crate::helpers;
use crate::minimax;
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
    pub two_players: bool,
}

impl Game {
    pub fn init(&mut self) {
        self.current_turn = Player::Cross;
        self.board = [[' '; 3]; 3];
        self.is_draw = false;
    }

    pub fn play_turn(&mut self, row: usize, col: usize) {
        match self.current_turn {
            Player::Circle => self.board[row][col] = 'O',
            Player::Cross => self.board[row][col] = 'X',
        }
    }

    pub fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        }
    }

    pub fn play_ai_turn(&mut self) {
        let mut best_score = -1000;

        let mut final_row: usize = 0;
        let mut final_col: usize = 0;

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == ' ' {
                    self.board[row][col] = 'X';

                    let score = minimax::minimax(self, false);

                    self.board[row][col] = ' ';

                    if score > best_score {
                        best_score = score;
                        final_row = row;
                        final_col = col;
                    }
                }
            }
        }

        self.board[final_row][final_col] = 'X';
    }

    fn set_winner(&mut self, row: usize, col: usize) {
        self.winner = match self.board[row][col] {
            'X' => Player::Cross,
            'O' => Player::Circle,
            _ => Player::Circle,
        };
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

    pub fn is_board_filled(&self) -> bool {
        for row in self.board {
            for col in row {
                if col == ' ' {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_game_over(&mut self) -> bool {
        let board_filled = self.is_board_filled();

        for i in 0..3 {
            if helpers::equal3(self.board[i][0], self.board[i][1], self.board[i][2]) {
                self.set_winner(i, 0);
                return true;
            }

            if helpers::equal3(self.board[0][i], self.board[1][i], self.board[2][i]) {
                self.set_winner(0, i);
                return true;
            }
        }

        if helpers::equal3(self.board[0][0], self.board[1][1], self.board[2][2]) {
            self.set_winner(0, 0);
            return true;
        }

        if helpers::equal3(self.board[0][2], self.board[1][1], self.board[2][0]) {
            self.set_winner(0, 2);
            return true;
        }

        if board_filled {
            self.is_draw = true;
            return true;
        } else {
            self.is_draw = false;
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
