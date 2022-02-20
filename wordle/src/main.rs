use std::{io, process};

use helpers::clear_line;

mod constants;
mod helpers;

static WORD: &str = "CHASE";

fn main() {
    process::Command::new("clear").status().unwrap();

    helpers::color_print(
        constants::RED,
        format!("The word is {}", String::from(WORD)),
        true,
    );

    let mut guess_number = 0;

    while guess_number < 5 {
        let mut user_input = String::from("");
        io::stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.to_string().trim().to_uppercase();

        clear_line();

        helpers::print_color_guesses(user_input, String::from(WORD));

        guess_number += 1;
    }
}
