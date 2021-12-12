use crate::game;

const MAX_PLAYER: char = 'X';
const MIN_PLAYER: char = 'O';
const EMPTY: char = ' ';

fn min(a: i32, b: i32) -> i32 {
    return if a < b { a } else { b };
}

fn max(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}

pub fn minimax(game: &mut game::Game, is_maximizing_player: bool) -> i32 {
    // considering O as the maximizing player

    if game.is_game_over() {
        if game.is_draw {
            return 0;
        }

        if game.winner == game::Player::Circle {
            return -10;
        } else {
            return 10;
        }
    }

    if is_maximizing_player {
        let mut best_score = -1000;

        for row in 0..3 {
            for col in 0..3 {
                if game.board[row][col] == EMPTY {
                    game.board[row][col] = MAX_PLAYER;

                    let score = minimax(game, false);

                    game.board[row][col] = EMPTY;

                    best_score = max(score, best_score);
                }
            }
        }

        return best_score;
    } else {
        let mut best_score = 1000;

        for row in 0..3 {
            for col in 0..3 {
                if game.board[row][col] == EMPTY {
                    game.board[row][col] = MIN_PLAYER;

                    let score = minimax(game, true);

                    game.board[row][col] = EMPTY;

                    best_score = min(score, best_score);
                }
            }
        }

        return best_score;
    }
}
