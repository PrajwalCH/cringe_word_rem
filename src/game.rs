use std::io::{self, Write};

enum Command {
    Quit,
    Skip,
    NotFound
}

pub fn start_loop(words_vec: &Vec<String>) {
    // game loop
    for word in words_vec.iter() {
        print_cringe_word(&word);

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


fn print_cringe_word(word: &str) {
    let cringe_word = make_cringe_word(&word);

    println!("");
    println!("What is the correct form of '{}'", cringe_word);
}

fn make_cringe_word(word: &str) -> String {
    let mut cringe_word: Vec<char> = word.chars().collect();
    fastrand::shuffle(&mut cringe_word);

    cringe_word.iter().collect()
}

fn take_user_guess() -> String {
    let mut user_guess = String::new();

    print!("> ");

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
