use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use std::option::Option;
use std::cmp::Ordering;

use fastrand;

#[derive(Eq, PartialEq)]
enum Command {
    Quit,
    Skip,
    NotFound
}

pub fn start() {
    let mut words_vec = match read_words_from_file() {
        Some(vec) => vec,
        None => {
            println!("Failed to read word data file");
            return
        }
    };
    fastrand::shuffle(&mut words_vec);

    // game loop
    for word in words_vec.iter() {
        let cringe_word = make_cringe_word(&word);

        println!("What is the correct form of '{}'", cringe_word);
        print!("> ");

        let user_guess = take_user_guess();
        let cmd = is_command(&user_guess);

        if cmd != Command::NotFound {
            if cmd == Command::Quit {
                // TODO: update the score (current)
                break;
            }

            if cmd == Command::Skip {
                // TODO: update the score (decrease)
                continue;
            }
        }

        println!("Your guess: {}", user_guess);
    }
}

fn read_words_from_file() -> Option<Vec<String>> {
    let mut words_vec: Vec<String> = Vec::new();

    let file = match File::open("words_data.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {:#?}", e);
            return None;
        }
    };
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();

        if line.trim().len() == 0 {
            continue;
        }
        words_vec.push(line);
    }

    Some(words_vec)
}

fn make_cringe_word(word: &str) -> String {
    let mut cringe_word: Vec<char> = word.clone().chars().collect();
    fastrand::shuffle(&mut cringe_word);

    let cringe_word: String = cringe_word.into_iter().collect();

    cringe_word
}

fn take_user_guess() -> String {
    let mut user_guess = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_guess).unwrap();

    user_guess.trim().to_string()
}

fn is_command(user_guess: &str) -> Command {
    if user_guess.cmp("quit") == Ordering::Equal {
        return Command::Quit;
    }

    if user_guess.cmp("skip") == Ordering::Equal {
        return Command::Skip;
    }

    Command::NotFound
}
