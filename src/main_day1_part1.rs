#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let a = read_lines("./input_day1.txt").unwrap();
    let mut most_cals: i32 = 0;
    let mut sum: i32 = 0;
    for i in a {
        let n: i32 = i.unwrap().parse().unwrap_or(0);
        if n > 0 {
            sum += n;
        } else {
            println!(">> {}", sum);
            if sum > most_cals {
                most_cals = sum;
            }
            sum = 0;
        }
    }
    println!("most cals:{}", most_cals);
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
