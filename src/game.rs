use std::io::{BufRead, BufReader};
use std::fs::File;
use std::option::Option;

pub fn start() {
    let mut words_vec = match read_words_file() {
        Some(vec) => vec,
        None => {
            println!("Failed to read word data file");
            return
        }
    };

    for word in words_vec.iter() {
        println!("{}", word);
    }
}

fn read_words_file() -> Option<Vec<String>> {
    let mut words_vec: Vec<String> = Vec::new();

    let file = match File::open("word_dict.txt") {
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
