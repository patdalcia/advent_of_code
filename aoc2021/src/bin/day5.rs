use aoc2021::get_puzzle;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    /// Returns true if the line from `self` to `other` is exactly at 45° diagonal,
    fn is_diagonal(&self, other: &Point) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx == dy
    }
}

fn parse_coord(s: &str) -> Point {
    let mut parts = s.split(',');
    let x_str = parts.next().expect("Missing x");
    let y_str = parts.next().expect("Missing y");
    let x = x_str
        .trim()
        .parse::<u32>()
        .expect("ERROR: Could not parse x");
    let y = y_str
        .trim()
        .parse::<u32>()
        .expect("ERROR: Could not parse y");
    Point { x, y }
}

fn solve_puzzle1(input: &str) -> (usize, usize) {
    let mut walls_part_one: HashMap<Point, u32> = HashMap::new();
    let mut walls_part_two: HashMap<Point, u32> = HashMap::new();
    for line in input.lines() {
        let points: Vec<Point> = line
            .split("->")
            .map(|split_line| parse_coord(split_line.trim()))
            .collect();
        for pair in points.chunks(2) {
            if pair.len() < 2 {
                continue;
            }
            let start = &pair[0];
            let end = &pair[1];

            if start.y == end.y {
                let x0 = start.x.min(end.x);
                let x1 = start.x.max(end.x);
                for x in x0..=x1 {
                    let current_coord = Point { x, y: start.y };
                    check_map(&mut walls_part_one, &current_coord);
                    check_map(&mut walls_part_two, &current_coord);
                }
            } else if start.x == end.x {
                let y0 = start.y.min(end.y);
                let y1 = start.y.max(end.y);
                for y in y0..=y1 {
                    let current_coord = Point { x: start.x, y };
                    check_map(&mut walls_part_one, &current_coord);
                    check_map(&mut walls_part_two, &current_coord);
                }
            } else if start.is_diagonal(end) {
                let dx = if end.x > start.x { 1i32 } else { -1i32 };
                let dy = if end.y > start.y { 1i32 } else { -1i32 };
                let steps = start.x.abs_diff(end.x) as i32;
                for i in 0..=steps {
                    let cx = (start.x as i32 + i * dx) as u32;
                    let cy = (start.y as i32 + i * dy) as u32;
                    let current_coord = Point { x: cx, y: cy };
                    check_map(&mut walls_part_two, &current_coord);
                }
            }
        }
    }
    let answer_part_one = walls_part_one
        .iter()
        .filter(|&(_, &count)| count > 1)
        .count();

    let answer_part_two = walls_part_two
        .iter()
        .filter(|&(_, &count)| count > 1)
        .count();
    (answer_part_one, answer_part_two)
}

fn check_map(walls: &mut HashMap<Point, u32>, p: &Point) -> u32 {
    let counter = walls.entry(p.clone()).or_insert(0);
    *counter += 1;
    *counter
}

fn main() {
    match get_puzzle("inputs/5.txt") {
        Ok(input) => {
            let answer = solve_puzzle1(input.as_str());
            println!(
                "ANSWER TO PART ONE: {}\nANSWER TO PART TWO: {}",
                answer.0, answer.1
            );
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
