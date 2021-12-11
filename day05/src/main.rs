use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line(Point, Point);

fn parse_line(line: &str) -> Line {
    let rgx = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let matches = rgx.captures(line).unwrap();

    let x1 = matches.get(1).unwrap().as_str().parse().unwrap();
    let y1 = matches.get(2).unwrap().as_str().parse().unwrap();
    let x2 = matches.get(3).unwrap().as_str().parse().unwrap();
    let y2 = matches.get(4).unwrap().as_str().parse().unwrap();

    return Line(Point(x1, y1), Point(x2, y2));
}

fn filter_straight(line: &Line) -> bool {
    return line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1;
}

fn get_bounds(lines: &[Line]) -> Point {
    let max_x = lines
        .iter()
        .max_by_key(|a| cmp::max(a.0 .0, a.1 .0))
        .unwrap();

    let max_y = lines
        .iter()
        .max_by_key(|a| cmp::max(a.0 .1, a.1 .1))
        .unwrap();

    return Point(
        cmp::max(max_x.0 .0, max_x.1 .0) + 1,
        cmp::max(max_y.0 .1, max_y.1 .1) + 1,
    );
}

// Shouldve read part 2 first, implemented generic  way of getting integer points of lines
fn get_points_on_line(line: &Line) -> Vec<Point> {
    let dx = line.1 .0 - line.0 .0;
    let dy = line.1 .1 - line.0 .1;
    let max_len = dx.abs() + dy.abs() + 1;
    let step;

    if dx == 0 {
        step = Point(0, dy.signum());
    } else if dy == 0 {
        step = Point(dx.signum(), 0);
    } else if ((dx as f32) / (dy as f32)).abs() < 1.0 {
        step = Point(dx.signum(), dy / dx.abs());
    } else {
        step = Point(dx / dy.abs(), dy.signum());
    }

    let mut points = Vec::new();
    let mut idx = 0;
    while idx < max_len {
        let point;
        if step.0 == 0 {
            let x = line.0 .0 + idx * step.0;
            let y = line.0 .1 + idx / step.1;
            point = Point(x, y);
        } else {
            let x = line.0 .0 + idx / step.0;
            let y = line.0 .1 + idx * step.1;
            point = Point(x, y);
        }

        points.push(point);

        if point == line.1 {
            break;
        }

        idx += 1;
    }

    return points;
}

fn count_intersects(lines: &[Line]) -> usize {
    let bounds = get_bounds(lines);
    let mut map = HashMap::new();

    for line in lines {
        for point in get_points_on_line(line) {
            let idx = point.1 * bounds.0 + point.0;

            if map.contains_key(&idx) {
                map.insert(idx, map.get(&idx).unwrap() + 1);
            } else {
                map.insert(idx, 1);
            }
        }
    }

    return map.values().filter(|cnt| **cnt > 1).count();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);

    let str_lines: Vec<String> = reader.lines().filter_map(std::io::Result::ok).collect();

    let lines: Vec<Line> = str_lines.iter().map(|line| parse_line(line)).collect();

    let straight_lines: Vec<Line> = lines
        .iter()
        .filter(|line| filter_straight(line))
        .map(|line| line.to_owned())
        .collect();

    println!("{}", count_intersects(&straight_lines));
    println!("{}", count_intersects(&lines));
}
