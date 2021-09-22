mod game;

use std::io::{self, Write};

fn main() {
    loop {
        print_menu();
        let user_choice = take_user_choice();

        match user_choice {
            1 => game::start(),
            2 => break,
            _ => {
                println!("Invalid menu option");
                continue
            },
        }
    }
}

fn print_menu() {
    println!("1. Start");
    println!("2. Exit");
}

fn take_user_choice() -> u8 {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read input");

    let user_choice: u8 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Oops!! only numbers are allowed: {:#?}", err);
            0
        }
    };

    user_choice
}
