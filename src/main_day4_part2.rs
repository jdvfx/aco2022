#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

fn main() {
    //
    let file = fs::read_to_string("./input_day4.txt").unwrap();
    let lines: Vec<_> = file.trim().split("\n").collect();

    let mut assignment_pairs: u32 = 0;

    for line in lines {
        // 
        let pair: Vec<&str> = line.split(",").collect();
        let elf1: Vec<&str> = pair[0].split("-").collect();
        let elf2: Vec<&str> = pair[1].split("-").collect();

        let elf1_min: u32 = elf1[0].parse().unwrap();
        let elf1_max: u32 = elf1[1].parse().unwrap();
        let elf2_min: u32 = elf2[0].parse().unwrap();
        let elf2_max: u32 = elf2[1].parse().unwrap();

        'contain: for i in elf1_min..=elf1_max {
            for j in elf2_min..=elf2_max {
                if i == j {
                    assignment_pairs += 1;
                    break 'contain;
                }
            }
        }
    }
    println!("assignment_pairs:{}", assignment_pairs);
}
