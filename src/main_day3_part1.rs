#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap; // 0.7.2

fn find_matching_char(string_a: &str, string_b: &str) -> Option<char> {
    for i in string_a.chars() {
        for j in string_b.chars() {
            if i == j {
                return Some(i);
            }
        }
    }
    None
}

fn letter_to_score(letter: char) -> i32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for (index, char_) in letters.chars().enumerate() {
        if char_ == letter {
            return (index + 1_usize) as i32;
        }
    }
    0
}

fn main() {
    let mut total_score = 0;

    // to map letters to items, outcome, and scores
    let lines = read_lines("./input_day3.txt").unwrap();

    for line in lines {
        let line = line.unwrap();

        let mid: f64 = line.len() as f64 * 0.5;
        let mid: usize = mid as usize;
        let two_compartments = line.split_at(mid);

        match find_matching_char(two_compartments.0, two_compartments.1) {
            Some(c) => total_score += letter_to_score(c),
            None => (),
        }
    }
    println!("{}", total_score);
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
