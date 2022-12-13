#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

fn main() {
    //
    let file = fs::read_to_string("./input_day3.txt").unwrap();
    let lines: Vec<_> = file.trim().split("\n").collect();

    let mut total_score: u32 = 0;
    // get 3 elves at a time
    for three_elves in lines.chunks(3) {
        println!("... 3 elves chunk ... ");

        // per elf, collect all unique items
        let mut all_items: Vec<char> = Vec::new();

        for elf in three_elves {
            let mut items_with_elf: Vec<char> = elf.chars().collect();
            items_with_elf.sort();
            items_with_elf.dedup();
            let j: String = items_with_elf.iter().collect();
            println!("{} {} ", elf, j);
            all_items.extend(items_with_elf);
        }
        all_items.sort();
        let x: String = all_items.iter().collect();
        println!("{:?}", x);

        let mut letter = '_';
        'c: for item_i in &all_items {
            let mut count = 0;
            for item_j in &all_items {
                if item_j == item_i {
                    count += 1;
                    if count == 3 {
                        letter = *item_j;
                        break 'c;
                    }
                }
            }
        }
        //
        println!("letter: {}", letter);

        let score = letter_to_score(letter);
        total_score += score;
    }
    println!("{}", total_score);
}
fn letter_to_score(letter: char) -> u32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for (index, char_) in letters.chars().enumerate() {
        if char_ == letter {
            return (index + 1_usize) as u32;
        }
    }
    0
}
