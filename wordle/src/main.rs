use std::process;

use helpers::{clear_line, color_print, get_random_word, print_color_guesses, take_user_input};

mod constants;
mod helpers;

fn main() {
    process::Command::new("clear").status().unwrap();

    #[allow(non_snake_case)]
    let WORD = get_random_word();

    // color_print(
    //     constants::RED,
    //     format!("The word is {}", String::from(WORD)),
    //     true,
    // );

    let mut guess_number = 0;

    while guess_number < 5 {
        let user_input = take_user_input();

        if user_input.len() != 5 {
            color_print(
                constants::RED,
                String::from("Length of input must be 5 characters"),
                true,
            );
            continue;
        }

        clear_line();

        print_color_guesses(&user_input, String::from(WORD));

        if user_input == WORD {
            break;
        }

        guess_number += 1;
    }

    color_print(
        constants::GREEN,
        format!("\nThe word was {}", String::from(WORD)),
        true,
    );
}
