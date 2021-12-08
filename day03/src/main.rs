use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let bitmaps: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Should not happen"),
                })
                .collect()
        })
        .collect();

    println!("{:?}", bitmaps);

    let bit_length = bitmaps[0].len();
    let total_count = bitmaps.len();
    let count_ones = bitmaps.iter().fold(vec![0; bit_length], |acc, x| {
        acc.iter().zip(x).map(|(a, b)| a + b).collect()
    });

    println!("{:?}", count_ones);

    let gamma_epsilon_bits: Vec<(i32, i32)> = count_ones
        .iter()
        .map(|bit| {
            if bit > &((total_count as i32) / 2) {
                return (1, 0);
            } else {
                return (0, 1);
            }
        })
        .collect();

    println!("{:?}", gamma_epsilon_bits);

    let (gamma, epsilon) = gamma_epsilon_bits
        .iter()
        .fold((0, 0), |(g_acc, e_acc), (g_bit, e_bit)| {
            ((g_acc << 1) + g_bit, (e_acc << 1) + e_bit)
        });

    println!("{}, {}", gamma, epsilon);
    println!("{}", gamma * epsilon);

    let mut oxy_rating_index = 0;
    let mut oxy_rating_bitmaps = bitmaps.clone();

    while oxy_rating_bitmaps.len() > 1 {
        let bit_length = oxy_rating_bitmaps[0].len();
        let total_count = oxy_rating_bitmaps.len();
        let count_ones = oxy_rating_bitmaps
            .iter()
            .fold(vec![0; bit_length], |acc, x| {
                acc.iter().zip(x).map(|(a, b)| a + b).collect()
            });

        oxy_rating_bitmaps = oxy_rating_bitmaps
            .iter()
            .filter(|bitmap| {
                if (count_ones[oxy_rating_index] as f32) >= (total_count as f32) / 2.0
                    && bitmap[oxy_rating_index] == 1
                {
                    return true;
                } else if (count_ones[oxy_rating_index] as f32) < (total_count as f32) / 2.0
                    && bitmap[oxy_rating_index] == 0
                {
                    return true;
                }

                return false;
            })
            .map(|bitmap| bitmap.to_owned())
            .collect();

        oxy_rating_index += 1;
    }

    let oxy_rating = oxy_rating_bitmaps[0]
        .iter()
        .fold(0, |acc, bit| (acc << 1) + bit);

    println!("{}", oxy_rating);

    let mut co2_rating_index = 0;
    let mut co2_rating_bitmaps = bitmaps.clone();

    while co2_rating_bitmaps.len() > 1 {
        let bit_length = co2_rating_bitmaps[0].len();
        let total_count = co2_rating_bitmaps.len();
        let count_ones = co2_rating_bitmaps
            .iter()
            .fold(vec![0; bit_length], |acc, x| {
                acc.iter().zip(x).map(|(a, b)| a + b).collect()
            });

        co2_rating_bitmaps = co2_rating_bitmaps
            .iter()
            .filter(|bitmap| {
                if (count_ones[co2_rating_index] as f32) >= (total_count as f32) / 2.0
                    && bitmap[co2_rating_index] == 0
                {
                    return true;
                } else if (count_ones[co2_rating_index] as f32) < (total_count as f32) / 2.0
                    && bitmap[co2_rating_index] == 1
                {
                    return true;
                }

                return false;
            })
            .map(|bitmap| bitmap.to_owned())
            .collect();

        co2_rating_index += 1;
    }

    let co2_rating = co2_rating_bitmaps[0]
        .iter()
        .fold(0, |acc, bit| (acc << 1) + bit);

    println!("{}", co2_rating);

    println!("{}", oxy_rating * co2_rating);
}
