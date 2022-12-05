#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

// input
// A rock
// B paper
// C scissors

// response
// X rock
// Y paper
// Z scissors

// score per item
// 1 rock
// 2 paper
// 3 scissors

// score per round
// 0 lost
// 3 draw
// 6 won

// rock>scissors
// scissors>paper
// paper>rock
//
fn main() {
    let fight: Vec<(&str, i32, i32)> = vec![
        ("AX", 3, 1),
        ("BY", 3, 2),
        ("CZ", 3, 3),
        ("AY", 6, 2),
        ("AZ", 0, 3),
        ("BX", 0, 1),
        ("BZ", 6, 3),
        ("CY", 0, 2),
        ("CX", 6, 1),
    ];

    let lines = read_lines("./input_day2.txt").unwrap();

    let mut totalscore: i32 = 0;
    for i in lines {
        let line = i.unwrap();
        let a: String = line.chars().filter(|x| x.ne(&' ')).collect();
        let mut score = 0;
        for j in &fight {
            if j.0 == a {
                score = j.1 + j.2;
                break;
            }
        }
        totalscore += score;
    }
    println!("{totalscore}");
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
