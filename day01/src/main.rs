use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("file: {}", filename);

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let depths: Vec<i32> = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let num: i32 = line.parse().expect("Invalid number");
            num
        })
        .collect();

    let mut counter = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            counter += 1;
        }
    }

    println!("{}", counter);

    let mut depth1: Vec<i32> = depths.iter().copied().collect();
    depth1.extend([0, 0].iter().copied());

    let mut depth2: Vec<i32> = vec![0];
    depth2.extend(depths.iter().copied());
    depth2.push(0);

    let mut depth3: Vec<i32> = vec![0, 0];
    depth3.extend(depths.iter().copied());

    let three_depths_sum: Vec<i32> = depth1
        .iter()
        .zip(depth2)
        .zip(depth3)
        .map(|((x, y), z)| x + y + z)
        .skip(2)
        .take(depths.len())
        .collect();

    counter = 0;

    for i in 1..three_depths_sum.len() {
        if three_depths_sum[i] > three_depths_sum[i - 1] {
            counter += 1;
        }
    }

    println!("{}", counter);
}
