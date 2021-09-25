mod game;

use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::fs::File;
use std::path::Path;

pub fn start() {
    loop {
        print_menu();

        let user_choice = take_user_choice();

        match user_choice {
            1 => {
                match read_words_from_file("words_data.txt") {
                    Ok(mut data) => {
                        fastrand::shuffle(&mut data);
                        game::start_loop(&data);
                    },
                    Err(error) => {
                        if error.kind() == ErrorKind::NotFound {
                            println!("Words data file not found");
                            break;
                        } else {
                            println!("Failed to read words data file");
                            break;
                        }
                    }
               };
            },
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
        Err(_) => {
            println!("Oops!! only numbers are allowed");
            0
        }
    };

    user_choice
}

fn read_words_from_file(file_name: &str) -> io::Result<Vec<String>> {
    let home_path = Path::new(option_env!("HOME").unwrap_or("."));
    let file_path = home_path.join(file_name);

    let file = File::open(file_path)?;
    let file = BufReader::new(file);

    let words_vec: Vec<String> = file.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line)
            } else {
                None
            }
        }).collect();

    Ok(words_vec)
}
