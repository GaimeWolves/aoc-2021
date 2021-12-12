use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn do_iteration(fish: &mut HashMap<i64, i64>) {
    let tmp_0 = *fish.get(&0).unwrap();

    for i in 1..9 {
        fish.insert(i - 1, *fish.get(&i).unwrap());
    }

    fish.insert(6, *fish.get(&6).unwrap() + tmp_0);
    fish.insert(8, tmp_0);
}

fn iterate_until(fish: &mut HashMap<i64, i64>, days: i32) {
    for _ in 0..days {
        do_iteration(fish);
        //println!("{:?}", fish)
    }
}

fn get_sum(fish: &HashMap<i64, i64>) -> i64 {
    return fish.values().sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let str_lines: Vec<String> = reader.lines().filter_map(std::io::Result::ok).collect();

    let mut init_fish_list: Vec<i64> = str_lines[0]
        .split(',')
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    let mut fish: HashMap<i64, i64> = HashMap::new();

    for i in 0..9 {
        fish.insert(i, 0);
    }

    for cur_fish in init_fish_list {
        fish.insert(cur_fish, *fish.get(&cur_fish).unwrap() + 1);
    }

    println!("{:?}", fish);

    iterate_until(&mut fish, 80);

    println!("{:?}", get_sum(&fish));

    iterate_until(&mut fish, 256 - 80);

    println!("{:?}", get_sum(&fish));
}
