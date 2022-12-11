#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap; // 0.7.2

fn main() {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total_score = 0;

    // to map letters to items, outcome, and scores
    let lines = read_lines("./input_day3.txt").unwrap();

    for line in lines {
        let line = line.unwrap();

        let mid: f64 = line.len() as f64 * 0.5;
        let mid: usize = mid as usize;
        let two_compartments = line.split_at(mid);

        'outer: for i in two_compartments.0.chars() {
            for j in two_compartments.1.chars() {
                if i == j {
                    for (index, char_) in letters.chars().enumerate() {
                        if char_ == i {
                            total_score += (index + 1_usize) as i32;
                            println!("{} {}", char_, index);
                            break 'outer;
                        }
                    }
                }
            }
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
