use aoc2023::get_puzzle;

fn solve_puzzle(input: String) {
    input.lines().for_each(|line| {
        if let Some(first_num) = line.chars().find_map(|ch| ch.to_digit(10)) {
            if let Some(second_num) = line.chars().rev().find_map(|ch| ch.to_digit(10)) {
                println!("FIRST NUM: {first_num} SECOND NUM: {second_num}");
            }
        }
    });
}

fn main() {
    match get_puzzle("../inputs/1.txt") {
        Ok(input) => solve_puzzle(input),
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
