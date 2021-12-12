use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn find_max(crabs: &[i32]) -> i32 {
    return *crabs.iter().max().unwrap();
}

fn get_fuel_cost(length: i32) -> i32 {
    return (length * (length + 1)) / 2;
}

fn part1(crabs: &[i32]) -> i32 {
    let min = 0;
    let max = find_max(crabs);

    let range = min..(max + 1);

    return range
        .map(|pos| crabs.iter().fold(0, |acc, crab| acc + (crab - pos).abs()))
        .min()
        .unwrap();
}

fn part2(crabs: &[i32]) -> i32 {
    let min = 0;
    let max = find_max(crabs);

    let range = min..(max + 1);

    return range
        .map(|pos| {
            crabs
                .iter()
                .fold(0, |acc, crab| acc + get_fuel_cost((crab - pos).abs()))
        })
        .min()
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let str_lines: Vec<String> = reader.lines().filter_map(std::io::Result::ok).collect();

    let crabs: Vec<i32> = str_lines[0]
        .split(',')
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    println!("{}", part1(&crabs));
    println!("{}", part2(&crabs));
}
