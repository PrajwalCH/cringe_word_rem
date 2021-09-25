use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::fs::File;
use std::cmp::Ordering;
use std::path::Path;

#[derive(Eq, PartialEq)]
enum Command {
    Quit,
    Skip,
    NotFound
}

pub fn start() {
    let mut words_vec = match read_words_from_file("words_data.txt") {
        Ok(data) => data,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                println!("Words data not found");
                return
            } else {
                println!("Failed to read words data file");
                return
            }
        }
    };

    fastrand::shuffle(&mut words_vec);

    // game loop
    for word in words_vec.iter() {
        let cringe_word = make_cringe_word(&word);

        println!("");
        println!("What is the correct form of '{}'", cringe_word);
        print!("> ");

        let user_guess = take_user_guess();

        match parse_command(&user_guess) {
            Command::Quit => break,
            Command::Skip => continue,
            Command::NotFound => {
                if user_guess != *word {
                    println!("Oops!! you guessed wrong");
                    println!("Correct word is '{}'", word);
                    continue
                }

                println!("You guessed right :)");
            }
        }
    }
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

fn make_cringe_word(word: &str) -> String {
    let mut cringe_word: Vec<char> = word.chars().collect();
    fastrand::shuffle(&mut cringe_word);

    cringe_word.iter().collect()
}

fn take_user_guess() -> String {
    let mut user_guess = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_guess).unwrap();

    user_guess.trim().to_string()
}

fn parse_command(user_guess: &str) -> Command {
    if user_guess == "quit" {
        return Command::Quit;
    }

    if user_guess == "skip" {
        return Command::Skip;
    }

    Command::NotFound
}
