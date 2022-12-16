#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
use regex::Regex;
use std::fs;

fn main() {
    //
    let file = fs::read_to_string("./input_day5.txt").unwrap();
    let lines: Vec<_> = file.split("\n\n").collect();
    let positions = lines[0];
    let moves = lines[1];

    // stash positions in a vec of vec<char>
    let mut p: Vec<&str> = positions.split("\n").collect();
    p.pop();
    p.reverse();
    let mut v: Vec<Vec<char>> = Vec::new();
    for x in 0..9 {
        let mut k: Vec<char> = Vec::new();
        for i in &p {
            let c: Vec<char> = i.chars().collect();
            let cc = c.get(1 + x * 4);
            if cc.is_some() {
                let val = *cc.unwrap();
                if val.is_alphabetic() {
                    k.push(val);
                }
            }
        }
        v.push(k);
    }
    //
    let m: Vec<&str> = moves.trim().split("\n").collect();
    for i in m {
        let move_instructions: Vec<u32> = i
            .split(" ")
            .into_iter()
            .filter_map(|a| a.parse().ok())
            .collect();

        let mut move_from: usize = move_instructions[1].clone() as usize;
        let mut move_to: usize = move_instructions[2].clone() as usize;
        let moves: usize = move_instructions[0].clone() as usize;

        move_from -= 1;
        move_to -= 1;

        let mut crates_pile_to_move_from = v[move_from].clone();
        let mut crates_pile_to_move_to = v[move_to].clone();

        for move_ in 0..moves {
            let crate_to_move = crates_pile_to_move_from.pop();
            crates_pile_to_move_to.push(crate_to_move.unwrap());
        }
        v[move_from] = crates_pile_to_move_from;
        v[move_to] = crates_pile_to_move_to;
    }
    // display result after moves
    let mut top_crates: String = "".to_string();
    for i in &v {
        let x = i.clone().pop();
        top_crates.push_str(&x.unwrap().to_string());
    }
    print!("{}", top_crates);
}
