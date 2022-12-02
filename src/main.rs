#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elfcals: Vec<i32> = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    // for i in lines {
    //     println!("{:?}", i);
    // }
    let b: Vec<String> = lines.filter_map(|x| Some(x.unwrap())).collect();
    for i in b {
        println!("{:?}", i);
    }
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
