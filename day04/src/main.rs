use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn check_winning(board: &[(i32, bool)]) -> bool {
    for a in 0..5 {
        let mut winning_hor = true;
        let mut winning_ver = true;

        for b in 0..5 {
            let idx_hor = a * 5 + b; // horizontal
            let idx_ver = b * 5 + a; // vertical

            if !board[idx_hor].1 {
                winning_hor = false;
            }

            if !board[idx_ver].1 {
                winning_ver = false;
            }
        }

        if winning_hor || winning_ver {
            return true;
        }
    }

    return false;
}

fn mark_number(num: i32, board: &mut Vec<(i32, bool)>) {
    for (num_of_i, marked) in board.iter_mut() {
        if num_of_i == &num {
            *marked = true;
        }
    }
}

fn get_winning_board<'a>(boards: &'a [Vec<(i32, bool)>]) -> Option<&'a [(i32, bool)]> {
    for board in boards {
        if check_winning(board) {
            return Some(board.as_slice());
        }
    }

    return None;
}

fn get_last_board_winning(boards: &[Vec<(i32, bool)>]) -> Option<usize> {
    let mut losing_board: Option<usize> = None;

    for (i, board) in boards.iter().enumerate() {
        if !check_winning(board) {
            if losing_board.is_some() {
                return None;
            }

            losing_board = Some(i);
        }
    }

    return losing_board;
}

fn all_boards_winning(boards: &[Vec<(i32, bool)>]) -> bool {
    return boards.iter().all(|board| check_winning(board.as_slice()));
}

fn mark_all(num: i32, boards: &mut Vec<Vec<(i32, bool)>>) {
    for board in boards.iter_mut() {
        mark_number(num, board)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(std::io::Result::ok).collect();

    let mut nums_drawn: Vec<i32> = lines
        .iter()
        .take(1)
        .map(|line| {
            line.split(',')
                .filter_map(|num_str| num_str.parse().ok())
                .collect()
        })
        .next()
        .unwrap();

    // Parse all boards into arrays of numbers
    let board_strings: Vec<&String> = lines.iter().skip(2).collect();

    let mut boards: Vec<Vec<(i32, bool)>> = Vec::new();
    let mut current_board: Vec<(i32, bool)> = Vec::new();

    for (i, line) in board_strings.iter().enumerate() {
        if i % 6 == 5 {
            boards.push(current_board.clone());
            current_board.clear();
        } else {
            current_board.extend::<Vec<(i32, bool)>>(
                line.split_whitespace()
                    .filter_map(|num_str| num_str.parse().ok())
                    .map(|num| (num, false))
                    .collect(),
            )
        }
    }

    boards.push(current_board.clone());

    let mut last_num = 0;

    // Play until one board wins
    while get_winning_board(boards.as_slice()).is_none() && nums_drawn.len() > 0 {
        last_num = nums_drawn.remove(0);
        mark_all(last_num, &mut boards);
    }

    // Calculate part one
    let winning_board = get_winning_board(boards.as_slice()).unwrap();
    let sum_first: i32 = winning_board
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(num, _)| num)
        .sum();

    println!("{}", sum_first * last_num);

    // Play until only one board has not yet won
    while get_last_board_winning(boards.as_slice()).is_none() && nums_drawn.len() > 0 {
        last_num = nums_drawn.remove(0);
        mark_all(last_num, &mut boards);
    }

    // Get the losing board's id
    let losing_board = get_last_board_winning(boards.as_slice()).unwrap();

    // Play until it has won
    while !all_boards_winning(boards.as_slice()) && nums_drawn.len() > 0 {
        last_num = nums_drawn.remove(0);
        mark_all(last_num, &mut boards);
    }

    // Calculate part two
    let sum_last: i32 = boards[losing_board]
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(num, _)| num)
        .sum();

    println!("{}", sum_last * last_num);
}
