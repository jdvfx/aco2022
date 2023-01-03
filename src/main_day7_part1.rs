#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

fn main() {
    let stream = fs::read_to_string("./input_day7.txt").unwrap();

    let mut a = stream.split("$").collect::<Vec<_>>();
    a.remove(0);
    for i in a {
        let mut x = i.trim().split("\n").collect::<Vec<_>>();
        let y = x[0].split_whitespace().collect::<Vec<_>>();

        match y[0] {
            "ls" => {
                x.remove(0);
                let files = x
                    .iter()
                    .filter(|x| !x.starts_with("dir"))
                    .collect::<Vec<_>>();
                println!("ls: {:?}", files);
            }
            "cd" => println!("cd: {:?}", y[1]),
            _ => panic!("NOOOOOOOO!!!"),
        }
    }
}
