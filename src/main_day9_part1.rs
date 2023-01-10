#![allow(
    dead_code,
    unused_variables,
    unused_assignments,
    unused_imports,
    unused
)]

use std::cmp::max;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn display_rope(head_pos: &Pos, tail_pos: &Pos) {
    let mut v: Vec<Vec<char>> = Vec::new();
    let size_x = 6;
    let size_y = 5;
    let mut ss: Vec<String> = Vec::new();
    for y in 0..size_y {
        let mut s = "".to_string();
        for x in 0..size_x {
            if x == head_pos.x && y == head_pos.y {
                s.push_str("H");
            } else if x == tail_pos.x && y == tail_pos.y {
                s.push_str("T");
            } else {
                s.push_str(".");
            }
        }
        ss.push(s);
    }
    for i in ss {
        println!("{:?}", i);
    }
    println!(" ");
}

fn main() {
    let stream = fs::read_to_string("./input_day9.txt").unwrap();

    // === test DATA ===
//     let stream = "R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2"
//     .to_string();

    let x = stream.trim().split("\n").collect::<Vec<_>>();

    let mut head_pos: Pos = Pos { x: 0, y: 4 };
    let mut tail_pos: Pos = Pos { x: 0, y: 4 };

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    tail_positions.insert((tail_pos.x, tail_pos.y));

    println!("== Initial State == ");
    display_rope(&head_pos, &tail_pos);

    for i in x {
        println!("== {} == ", i);
        let s = i.split_whitespace().collect::<Vec<_>>();
        // println!("s:{:?}", s);
        let direction = s[0];
        let moves: i32 = s[1].to_string().parse::<i32>().unwrap();

        for mv in 0..moves {
            match direction {
                "D" => head_pos.y += 1,
                "U" => head_pos.y -= 1,
                "L" => head_pos.x -= 1,
                "R" => head_pos.x += 1,
                _ => (),
            }

            let diffx = (head_pos.x - tail_pos.x).abs();
            let diffy = (head_pos.y - tail_pos.y).abs();

            let dif = diffx + diffy;

            if diffx == 2 && diffy == 0 || diffy == 2 && diffx == 0 {
                match direction {
                    "D" => tail_pos.y += 1,
                    "U" => tail_pos.y -= 1,
                    "L" => tail_pos.x -= 1,
                    "R" => tail_pos.x += 1,
                    _ => (),
                }
            } else if diffx + diffy > 2 {
                tail_pos.x = head_pos.x;
                tail_pos.y = head_pos.y;
                match direction {
                    "D" => tail_pos.y -= 1,
                    "U" => tail_pos.y += 1,
                    "L" => tail_pos.x += 1,
                    "R" => tail_pos.x -= 1,
                    _ => (),
                }
            }

            display_rope(&head_pos, &tail_pos);
            tail_positions.insert((tail_pos.x, tail_pos.y));
        }
    }
    for i in &tail_positions {
        println!("{:?}", i);
    }
    println!("{}", tail_positions.len());
}
