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
    // 35390".to_string();
    // let x = g.split("\n").collect::<Vec<_>>();

    // parse file into vec of vec<u8>
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
    // loop through all cells
    for y in 1..dim - 1 {
        for x in 1..dim - 1 {
            // the current tree
            let tallest = trees[y][x];
            //
            let mut left = 0;
            for xx in (0..x).rev() {
                let val = trees[y][xx];
                left += 1;
                if val >= tallest {
                    break;
                }
            }
            let mut right = 0;
            for xx in x + 1..(dim) {
                right += 1;
                if trees[y][xx] >= tallest {
                    break;
                }
            }
            let mut top = 0;
            for yy in (0..y).rev() {
                top += 1;
                if trees[yy][x] >= tallest {
                    break;
                }
            }
            let mut bottom = 0;
            for yy in y + 1..(dim) {
                bottom += 1;
                if trees[yy][x] >= tallest {
                    break;
                }
            }
            let prod = top * bottom * right * left;
            // println!(
            //     "x:{},y:{} t:{} l:{} r:{} t:{} b:{} *{}",
            //     x, y, tallest, left, right, top, bottom, prod
            // );
            max_tree_visible = max(max_tree_visible, prod);
        }
    }
    println!("max tree visible {}", max_tree_visible);
}
