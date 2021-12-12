mod colors;
mod game;
mod helpers;
mod minimax;

fn main() {
    let mut game: game::Game = game::Game {
        board: [[' '; 3]; 3],
        current_turn: game::Player::Circle,
        winner: game::Player::Circle,
        is_draw: false,
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

        let (row, col) =
            helpers::check_for_errors(&game, cell_number, &user_input, &mut errors_in_buffer);

        if row == -1 {
            continue;
        }

        game.play_turn(row as usize, col as usize);

        game.change_turn();

        game.play_ai_turn();

        if game.is_game_over() {
            let start_new_game = helpers::handle_game_over(&game);

            if !start_new_game {
                break;
            }

            game.init();
            continue;
        }

        game.change_turn();
    }
}
