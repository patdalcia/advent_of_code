use aoc2021::get_puzzle;

const STARTING_POINT: u32 = 0;

fn solve_puzzle1(input: &str) -> (u32, u32) {
    let mut horizontal_position = STARTING_POINT;
    let mut depth = STARTING_POINT;
    for line in input.trim().lines() {
        if let Some((command, num_as_str)) = line.split_once(' ') {
            if let Ok(num_parsed) = num_as_str.parse::<u32>() {
                match command {
                    "forward" => horizontal_position += num_parsed,
                    "down" => depth += num_parsed,
                    "up" => depth -= num_parsed,
                    _ => {
                        println!("ERROR: Unexpected COMMAND in solve_puzzle -> {command}")
                    }
                }
            }
        }
    }
    (horizontal_position, depth)
}

fn solve_puzzle2(input: &str) -> (u32, u32) {
    let mut horizontal_position = STARTING_POINT;
    let mut depth = STARTING_POINT;
    let mut aim = STARTING_POINT;
    for line in input.trim().lines() {
        if let Some((command, num_as_str)) = line.split_once(' ') {
            if let Ok(num_parsed) = num_as_str.parse::<u32>() {
                match command {
                    "forward" => {
                        horizontal_position += num_parsed;
                        depth += aim * num_parsed;
                    }
                    "down" => aim += num_parsed,
                    "up" => aim -= num_parsed,
                    _ => {
                        println!("ERROR: Unexpected COMMAND in solve_puzzle -> {command}")
                    }
                }
            }
        }
    }
    (horizontal_position, depth)
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            let answer1 = solve_puzzle1(input.as_str());
            let answer2 = solve_puzzle2(input.as_str());
            println!("ANSWER TO PART ONE: {}", answer1.0 * answer1.1);
            println!("ANSWER TO PART TWO: {}", answer2.0 * answer2.1);
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
