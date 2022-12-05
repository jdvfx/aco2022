use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap; // 0.7.2

fn rps2(o: &str, r: i32) -> &str {
    let win_lose: Vec<(&str, &str)> = vec![
        ("rock", "scissors"),
        ("scissors", "paper"),
        ("paper", "rock"),
    ];

    for i in &win_lose {
        if r == 0 {
            if i.0 == o {
                // LOSE
                return i.1;
            }
        } else if r == 6 {
            // WIN
            if i.1 == o {
                return i.0;
            }
        }
    }
    o // DRAW
}

fn main() {
    // to map letters to items, outcome, and scores
    let mapitems = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
        ("X", "lose"),
        ("Y", "draw"),
        ("Z", "win"),
    ]);
    let gameresult = HashMap::from([("win", 6), ("draw", 3), ("lose", 0)]);
    let itemvalues = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);

    let lines = read_lines("./input_day2.txt").unwrap();

    let mut totalscore: i32 = 0;
    for line in lines {
        let line = line.unwrap();

        let opponent_item = mapitems[&line[0..1]]; // A,B,C > rock,paper,scissors
        let my_item = mapitems[&line[2..3]]; // X,Y,Z > win,draw,lose
        let gamescore = gameresult[my_item]; // win,draw,lose > 6,3,0

        let mymove = rps2(opponent_item, gamescore); // solve puzzle, what item ?
        let my_item_value = itemvalues[mymove]; // value of item

        totalscore += gamescore + my_item_value;
    }
    println!("{}", totalscore);
}

// from help
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
