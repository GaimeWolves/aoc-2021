use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Bit representations of the numbers
const REPRS: [i32; 10] = [
    0b01110111, 0b00100100, 0b01011101, 0b01101101, 0b00101110, 0b01101011, 0b01111011, 0b00100101,
    0b01111111, 0b01101111,
];

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sample {
    samples: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DeductionMatrix {
    wires: Vec<i32>,
}

impl Default for DeductionMatrix {
    fn default() -> Self {
        DeductionMatrix {
            wires: vec![
                0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111,
            ],
        }
    }
}

fn char_to_bit(chr: &char) -> i32 {
    return 1 << ((*chr as i32) - ('a' as i32));
}

fn parse_word(word: &str) -> i32 {
    let mut bitfield = 0;

    for chr in word.chars() {
        bitfield |= char_to_bit(&chr);
    }

    return bitfield;
}

fn parse_line(line: &str) -> Sample {
    let rgx = Regex::new(
        r"^(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) \| (\w+) (\w+) (\w+) (\w+)$",
    )
    .unwrap();
    let matches = rgx.captures(line).unwrap();

    let values: Vec<i32> = (1..15)
        .map(|idx| parse_word(matches.get(idx).unwrap().as_str()))
        .collect();

    return Sample { samples: values };
}

fn part1(samples: &[Sample]) -> i32 {
    return samples.iter().fold(0, |acc, sample| {
        acc + sample
            .samples
            .iter()
            .skip(10)
            .filter(|num| {
                let ones = num.count_ones();
                (ones >= 2 && ones <= 4) || ones == 7
            })
            .count() as i32
    });
}

fn get_samples_of_count(count: u32, sample: &Sample) -> Vec<&i32> {
    return sample
        .samples
        .iter()
        .filter(|sample| sample.count_ones() == count)
        .collect();
}

fn get_indices_from_sample(sample: i32) -> Vec<usize> {
    return (0..8).filter(|idx| (sample >> idx) & 1 == 1).collect();
}

fn deduce(sample: &Sample) -> DeductionMatrix {
    let mut matrix: DeductionMatrix = Default::default();

    // Pass 1: Restrict wires f and c from 1
    {
        let sample_1 = get_samples_of_count(2, sample)[0];
        let indices = get_indices_from_sample(*sample_1);

        for idx in indices {
            matrix.wires[idx] &= REPRS[1];
        }
    }

    // Pass 2: Deduce wire a from 7
    {
        let sample_7 = get_samples_of_count(3, sample)[0];
        let indices = get_indices_from_sample(*sample_7);

        for idx in indices {
            if matrix.wires[idx] & (1 << 0) > 0 {
                matrix.wires[idx] = 1 << 0;

                for i in 0..7 {
                    if i == idx {
                        continue;
                    }

                    matrix.wires[i] &= !(1 << 0);
                }
            }
        }
    }

    // Pass 3: Restrict wires d and b from 4
    {
        let sample_4 = get_samples_of_count(4, sample)[0];
        let indices = get_indices_from_sample(*sample_4);

        for idx in indices {
            if matrix.wires[idx] & (1 << 1) > 0 {
                matrix.wires[idx] = (1 << 3) | (1 << 1);
            }
        }
    }

    // Pass 4: Solve all except g using 0, 6, 9
    {
        let sample_069 = get_samples_of_count(6, sample);
        let indices_tmp: Vec<Vec<usize>> = sample_069
            .iter()
            .map(|sample| get_indices_from_sample(**sample))
            .collect();

        // Filter the indices in question (the ones that differ between 0, 6 and 9)
        let indices: Vec<usize> = (0..7)
            .filter(|idx| {
                indices_tmp.iter().filter(|idxs| idxs.contains(idx)).count() < sample_069.len()
            })
            .collect();

        // Pass 4.a: Find which only has one possibility
        for idx in indices.iter() {
            if matrix.wires[*idx] & (1 << 4) > 0 {
                matrix.wires[*idx] = 1 << 4;

                for i in 0..7 {
                    if i == *idx {
                        continue;
                    }

                    matrix.wires[i] &= !(1 << 4);
                }
            }
        }

        // Pass 4.b: Assign the other two bits
        for idx in indices.iter() {
            if matrix.wires[*idx] & (1 << 3) > 0 {
                matrix.wires[*idx] = 1 << 3;

                for i in 0..7 {
                    if i == *idx {
                        continue;
                    }

                    matrix.wires[i] &= !(1 << 3);
                }
            } else if matrix.wires[*idx] & (1 << 2) > 0 {
                matrix.wires[*idx] = 1 << 2;

                for i in 0..7 {
                    if i == *idx {
                        continue;
                    }

                    matrix.wires[i] &= !(1 << 2);
                }
            }
        }
    }

    // Pass 5: Find last remaining wire and assign it to wire g
    {
        for i in 0..7 {
            if matrix.wires[i] > 32 {
                matrix.wires[i] = 64;
                break;
            }
        }
    }

    return matrix;
}

fn repr_to_digit(repr: i32) -> i32 {
    for i in 0..10 {
        if repr == REPRS[i] {
            return i as i32;
        }
    }

    panic!("Should not happen");
}

fn get_output_value(sample: &Sample) -> i32 {
    let matrix = deduce(sample);

    return sample
        .samples
        .iter()
        .skip(10)
        .map(|num| {
            let mut repr = 0;

            for i in 0..7 {
                let bit = (num >> i) & 1;

                if bit == 1 {
                    repr |= matrix.wires[i];
                }
            }

            repr_to_digit(repr)
        })
        .fold(0, |acc, num| acc * 10 + num);
}

fn part2(samples: &[Sample]) -> i32 {
    return samples
        .iter()
        .fold(0, |acc, sample| acc + get_output_value(sample));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let str_lines: Vec<String> = reader.lines().filter_map(std::io::Result::ok).collect();
    let samples: Vec<Sample> = str_lines.iter().map(|line| parse_line(line)).collect();

    println!("{:?}", part1(&samples));
    println!("{:?}", part2(&samples));
}
