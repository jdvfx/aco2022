#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
use std::fs;

use raylib::prelude::*;
use std::collections::HashSet;

use std::{thread, time};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let amount_of_knots: usize = 9;

    // raylib init
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Hello, World")
        .build();
    let bg_color = Color::new(10, 10, 0, 255);
    let rec_color = Color::new(255, 255, 0, 255);
    let scale: i32 = 4;
    // let mut rng = rand::thread_rng();
    rl.set_exit_key(Some(KeyboardKey::KEY_A));
    // Start state
    rl.begin_drawing(&thread).clear_background(bg_color);
    rl.set_target_fps(120);

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    // screen offset
    let mut ox: i32 = 16;
    let mut oy: i32 = 16;

    let mut rope: Vec<Point> = Vec::new();
    for i in 0..=amount_of_knots {
        let p = Point { x: 0, y: 0 };
        rope.push(p);
    }

    let stream = fs::read_to_string("./adventofcode.com_2022_day_9_input.txt").unwrap();
    let m = stream.trim().split("\n").collect::<Vec<_>>();
    let mut g = m.iter();

    let mut mov = 0;
    let mut direction = "U";
    let mut steps: isize = 1;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let a = 0;
        if a == 0 {
            // if d.is_key_released(KeyboardKey::KEY_Q) {
            if mov == 0 {
                let gg = g.next();
                if gg.is_none() {
                    let ten_millis = time::Duration::from_secs(10);
                    thread::sleep(ten_millis);
                    panic!(">>> tail positions: {}", tail_positions.len());
                }
                let g2 = gg.unwrap().split_whitespace().collect::<Vec<_>>();
                // // number of moves in which direction
                steps = g2[1].parse().unwrap();
                direction = g2[0];
            }

            // move head
            move_(&mut rope[0], direction);
            // move knots
            for knot_id in 1..=amount_of_knots {
                let prev_knot = rope[knot_id - 1];
                let knot = rope[knot_id];

                let diffx = prev_knot.x - knot.x;
                let diffy = prev_knot.y - knot.y;

                let mut k = knot.clone();

                if diffy.abs() > 1 || diffx.abs() > 1 {
                    k.y += diffy.signum();
                    k.x += diffx.signum();
                }

                if knot_id == amount_of_knots {
                    tail_positions.insert((k.x, k.y));
                }

                rope[knot_id] = k;
            }

            d.clear_background(bg_color);
            let c0 = Color::new(50, 0, 50, 255);
            d.draw_rectangle((0 - ox) * scale, (0 - oy) * scale, scale, scale, c0);

            for l in tail_positions.iter() {
                let c = Color::new(0, 40, 0, 255);
                d.draw_rectangle((l.0 - ox) * scale, (l.1 - oy) * scale, scale, scale, c);
            }

            for (i, k) in rope.iter().enumerate() {
                let c = match i {
                    0 => Color::new(255, 0, 0, 255),
                    9 => Color::new(0, 255, 0, 255),
                    _ => Color::new(0, i as u8 * 10, 255, 255),
                };
                d.draw_rectangle((k.x - ox) * scale, (k.y - oy) * scale, scale, scale, c);

                ox = k.x.min(ox);
                oy = k.y.min(oy);
            }

            // increment step
            if mov < steps - 1 {
                mov += 1;
            } else {
                mov = 0;
            }
        }

        for l in tail_positions.iter() {
            ox = l.0.min(ox);
            oy = l.1.min(oy);
        }
    }
}

fn move_(pos: &mut Point, direction: &str) {
    match direction {
        "D" => pos.y += 1,
        "U" => pos.y -= 1,
        "L" => pos.x -= 1,
        "R" => pos.x += 1,
        _ => (),
    }
}
