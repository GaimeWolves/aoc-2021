use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let instructions: Vec<(String, i32)> = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| -> _ {
            let parts = line
                .split_whitespace()
                .map(|str| -> _ { str.to_owned() })
                .take(2)
                .collect();
            parts
        })
        .map(|strs: Vec<String>| -> _ { (strs[0].to_owned(), strs[1].parse().expect("No number")) })
        .collect();

    let mut horizontal = 0;
    let mut depth = 0;

    println!("{:?}", instructions);

    for (instr, amount) in instructions.iter() {
        match instr.as_str() {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => (),
        }
    }

    let part1 = horizontal * depth;
    println!("{}", part1);

    let mut aim = 0;
    horizontal = 0;
    depth = 0;

    for (instr, amount) in instructions.iter() {
        match instr.as_str() {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => (),
        }
    }

    let part2 = horizontal * depth;
    println!("{}", part2);
}
