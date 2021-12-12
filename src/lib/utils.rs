use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input(test_num: &str) -> Vec<String> {
    let mut output = Vec::new();
    let file = File::open(format!("inputs/{}.txt", test_num)).unwrap();
    for line in BufReader::new(file).lines() {
        if let Ok(text) = line {
            output.push(text);
        }
    }
    output
}

