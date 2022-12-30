#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

fn main() {

    // convert string to bytes and windows of 4
    // remove duplicates, if len()==4, bingo.

    let stream = fs::read_to_string("./input_day6.txt").unwrap();

    let bytes = stream.into_bytes();
    let bytes_windows = bytes.windows(4);
    for (idx, window) in bytes_windows.enumerate() {
        let mut window_deduped = window.to_owned();
        window_deduped.sort();
        window_deduped.dedup();
        if window_deduped.len() == 4 {
            // convert bytes baco to string
            // to display the 4 letters (optional)
            let start_of_packet = String::from_utf8(window_deduped).unwrap();
            println!("{} {}", start_of_packet, idx + 4);
            break;
        }
    }
}
