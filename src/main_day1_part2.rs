#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elfcals: Vec<i32> = Vec::new();
    let a = read_lines("./input_day1.txt").unwrap();
    // let mut most_cals: i32 = 0;
    let mut sum: i32 = 0;
    for i in a {
        let n: i32 = i.unwrap().parse().unwrap_or(0);
        if n > 0 {
            sum += n;
        } else {
            elfcals.push(sum);
            sum = 0;
        }
    }
    elfcals.sort();
    elfcals.reverse();
    let aa = elfcals[0]+elfcals[1]+elfcals[2];
    println!("{:?}", aa);
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
