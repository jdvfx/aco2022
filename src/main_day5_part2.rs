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
    let mut positions_vec: Vec<&str> = positions.split("\n").collect();
    // remove line with numbers
    positions_vec.pop();
    positions_vec.reverse();
    let mut crates_position: Vec<Vec<char>> = Vec::new();
    for x in 0..9 {
        let mut k: Vec<char> = Vec::new();
        for i in &positions_vec {
            let c: Vec<char> = i.chars().collect();
            let cc = c.get(1 + x * 4);
            if cc.is_some() {
                let val = *cc.unwrap();
                if val.is_alphabetic() {
                    k.push(val);
                }
            }
        }
        crates_position.push(k);
    }

    //
    let moves_: Vec<&str> = moves.trim().split("\n").collect();

    // loop through lines of moves
    for current_move in moves_ {
        let move_instructions: Vec<u32> = current_move
            .split(" ")
            .into_iter()
            .filter_map(|a| a.parse().ok())
            .collect();

        let mut move_from: usize = move_instructions[1].clone() as usize;
        let mut move_to: usize = move_instructions[2].clone() as usize;
        let moves: usize = move_instructions[0].clone() as usize;

        move_from -= 1;
        move_to -= 1;

        // println!("{:?}",v.len());
        let from = &mut crates_position[move_from];
        let slice_end = from.len() - 1_usize;
        let slice_start = slice_end - (moves - 1);
        // let b: usize = 1;
        let slice = &from[slice_start..=slice_end];

        let crates_being_moved: Vec<_> = from.drain(slice_start..=slice_end).collect();
        let to = &mut crates_position[move_to];
        to.extend(crates_being_moved.iter());
    }

    // display result after moves
    let mut top_crates: String = "".to_string();
    for current_move in &crates_position {
        let top_crate = current_move.clone().pop();
        top_crates.push_str(&top_crate.unwrap().to_string());
    }
    print!("{}", top_crates);
}
