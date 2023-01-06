use std::cmp::max;
use std::fs;

fn main() {
    let stream = fs::read_to_string("./input_day8.txt").unwrap();
    let x = stream.trim().split("\n").collect::<Vec<_>>();

    // test DATA
    let g = "30373
    25512
    65332
    33549
    35390"
        .to_string();
    let x = g.split("\n").collect::<Vec<_>>();

    //
    let mut trees: Vec<Vec<u8>> = Vec::new();
    let mut viz_trees: Vec<Vec<u8>> = Vec::new();
    for i in &x {
        let v = i
            .chars()
            .filter_map(|x| x.to_string().parse::<u8>().ok())
            .collect::<Vec<u8>>();
        let viz = vec![0; x.len()];
        trees.push(v);
        viz_trees.push(viz);
    }
    let dim = viz_trees.len();

    // set borders to visible/1
    for y in 0..dim {
        for x in 0..dim {
            if x == 0 || y == 0 || x == dim - 1 || y == dim - 1 {
                viz_trees[y][x] = 1;
            }
        }
    }

    // top to bottom
    for y in 1..dim - 1 {
        let mut tallest = trees[y][0]; // LEFT
                                       // left to right
        for x in 1..dim - 1 {
            let val = trees[y][x];
            if val > tallest {
                viz_trees[y][x] = 1;
                tallest = max(tallest, val);
            }
        }
        let mut tallest = trees[y][dim - 1]; // RIGHT
                                             // right to left
        for x in (1..dim - 1).rev() {
            let val = trees[y][x];
            if val > tallest {
                viz_trees[y][x] = 1;
                tallest = max(tallest, val);
            }
        }
    }
    // left to right
    for x in 1..dim - 1 {
        let mut tallest = trees[0][x]; // TOP
                                       // top to bottom
        for y in 1..dim - 1 {
            let val = trees[y][x];
            if val > tallest {
                viz_trees[y][x] = 1;
                tallest = max(tallest, val);
            }
        }
        let mut tallest = trees[dim - 1][x]; // BOTTOM
                                             // bottom to top
        for y in (1..dim - 1).rev() {
            let val = trees[y][x];
            if val > tallest {
                viz_trees[y][x] = 1;
                tallest = max(tallest, val);
            }
        }
    }

    // display trees, visible trees, and sim of visible trees
    for y in &trees {
        let s = y
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("{}", s);
    }
    println!("...");
    for y in &viz_trees {
        let s = y
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("{}", s);
    }
    println!("...");
    let sum: u32 = viz_trees.iter().flatten().map(|x| *x as u32).sum();
    println!("sum {}", sum);
}
