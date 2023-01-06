#![allow(
    dead_code,
    unused_variables,
    unused_assignments,
    unused_imports,
    unused_mut
)]

use std::cmp::max;
use std::fs;

fn main() {
    let stream = fs::read_to_string("./input_day8.txt").unwrap();
    let x = stream.trim().split("\n").collect::<Vec<_>>();

    // test DATA
    // let g = "30373
    // 25512
    // 65332
    // 33549
    // 35390"
    //     .to_string();
    // let x = g.split("\n").collect::<Vec<_>>();

    //
    let mut trees: Vec<Vec<u8>> = Vec::new();
    let mut viz_trees: Vec<Vec<u8>> = Vec::new();
    for i in x {
        let v = i
            .chars()
            .filter_map(|x| x.to_string().parse::<u8>().ok())
            .collect::<Vec<u8>>();
        let viz = vec![0; v.len()];
        trees.push(v);
        viz_trees.push(viz);
    }
    let dim = viz_trees.len();

    let mut max_tree_visible = 0;
    // top to bottom
    for y in 1..dim - 1 {
        for x in 1..dim - 1 {
            let tallest = trees[y][x];

            let mut left = 0;
            for xx in (0..x).rev() {
                println!("xx> {:?}",xx);
                let val = trees[y][xx];
                if val >= tallest {
                    left += 1;
                    break;
                } else {
                    left += 1;
                }
            }
            let mut right = 0;
            for xx in x + 1..(dim) {
                println!("xx< {:?}",xx);
                let val = trees[y][xx];
                if val >= tallest {
                    right += 1;
                    break;
                } else {
                    right += 1;
                }
            }
            let mut top = 0;
            for yy in (0..y).rev() {
                println!("yyd {:?}",yy);
                let val = trees[yy][x];
                if val >= tallest {
                    top += 1;
                    break;
                } else {
                    top += 1;
                }
            }
            let mut bottom = 0;
            for yy in y + 1..(dim) {
                println!("yyu {:?}",yy);
                let val = trees[yy][x];
                if val >= tallest {
                    bottom += 1;
                    break;
                } else {
                    bottom += 1;
                }
            }
            let prod = top * bottom * right * left;
            println!(
                "x:{},y:{} t:{} l:{} r:{} t:{} b:{} *{}",
                x, y, tallest, left, right, top, bottom, prod
            );
            max_tree_visible = max(max_tree_visible, prod);
        }
    }
    println!("max tree visible {}", max_tree_visible);
}
