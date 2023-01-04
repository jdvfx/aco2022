// #![allow(
//     dead_code,
//     unused_variables,
//     unused_assignments,
//     unused_imports,
//     unused_mut
// )]

use std::{collections::HashMap, fs};

fn main() {
    let stream = fs::read_to_string("./input_day7.txt").unwrap();

    let mut a = stream.split("$").collect::<Vec<_>>();
    a.remove(0);

    let mut filepath: Vec<String> = Vec::new();
    let mut pathsizes: HashMap<String, u32> = HashMap::new();

    for i in a {
        println!(">>>> {}", i);

        let x = i.trim().split("\n").collect::<Vec<_>>();
        let y = x[0].split_whitespace().collect::<Vec<_>>();

        match y[0] {
            "ls" => {

                // find all sizes and sum them
                let ls_output_string = x.join(" ");
                let dirsize = ls_output_string
                    .split(" ")
                    .filter_map(|x| x.parse::<u32>().ok())
                    .sum::<u32>();

                // create loop with all paths from end to /
                // // and add the total size to each, store in hashmap
                // /a/b/c
                // /a/b
                // /a
                // /
                let mut fp = filepath.clone();
                for _i in 0..filepath.len() {
                    let mut path = &fp.join("/")[1..];
                    if path.is_empty() {
                        path = "/";
                    }

                    let path = path.to_string();
                    let dirsize_mut = pathsizes.get_mut(&path);
                    if dirsize_mut.is_some() {
                        *dirsize_mut.unwrap() += dirsize;
                    } else {
                        pathsizes.insert(path.to_string(), dirsize);
                    }
                    fp.pop();
                }
            }
            "cd" => {
                let dir = y[1];
                if dir == ".." {
                    filepath.pop();
                } else {
                    filepath.push(dir.to_string());
                }
                let path = &filepath.join("/")[1..];
                println!("cd: {:?}", path);
            }
            _ => panic!("NO WAY!!!"),
        }
    }
    // compute total size of all dirs with 100_000 or less
    let mut tot: u32 = 0;
    for i in pathsizes {
        let size = i.1;
        if size < 100000 {
            tot += size;
        }
    }
    println!("> > > {:?}", tot);
}
