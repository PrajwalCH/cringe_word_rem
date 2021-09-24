use std::io::{BufRead, BufReader};
use std::fs::File;
use std::option::Option;

use fastrand;

pub fn start() {
    let words_vec = match read_words_from_file() {
        Some(vec) => vec,
        None => {
            println!("Failed to read word data file");
            return
        }
    };

    let words_vec_idx_map = make_words_vec_idx_map(words_vec.len());

    for i in words_vec_idx_map.iter() {
        // save current word in game state
        // copy the current word in game state  and cringe it
        // ask user to guess and save guess word in game state
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

fn make_words_vec_idx_map(words_vec_len: usize) -> Vec<usize> {
    let mut words_vec_idx_map: Vec<usize> = Vec::with_capacity(words_vec_len);

    for i in 0..words_vec_len {
        words_vec_idx_map.push(i);
    }

    fastrand::shuffle(&mut words_vec_idx_map);
    words_vec_idx_map
}
