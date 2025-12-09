use aoc2025::get_puzzle;

const STARTING_POINT: i32 = 50;
const LOWER_LIMIT: i32 = 0;
const UPPER_LIMIT: i32 = 99;
const RANGE: i32 = UPPER_LIMIT - LOWER_LIMIT + 1;

fn parse_input(input: &str) -> Vec<(char, i32)> {
    println!("~~INPUT~~\n{input}");
    let mut instructions: Vec<(char, i32)> = Vec::new();
    for line in input.trim().lines() {
        if line.len() > 1 {
            let (first, rest) = line.split_at(1);
            if let Some(direction) = first.chars().next() {
                if let Ok(number) = rest.parse::<i32>() {
                    instructions.push((direction, number));
                }
            }
        }
    }
    instructions
}

fn wrap_around(pos: i32) -> i32 {
    let range = UPPER_LIMIT - LOWER_LIMIT + 1;
    let wrapped = ((pos % range) + range) % range;
    LOWER_LIMIT + wrapped
}

fn get_password(instructions: &Vec<(char, i32)>) -> i32 {
    let mut zero_counter = 0;
    let mut part_two_zero_counter = 0;
    let mut current_position: i32 = STARTING_POINT;
    for (direction, number) in instructions {
        match direction {
            'R' => {
                let mut offset = *number + current_position;
                if offset > UPPER_LIMIT {
                    offset /= RANGE;
                    println!("PASSED ZERO -> {offset} TIMES");
                    part_two_zero_counter += offset;
                }
                current_position += *number;
            }
            'L' => {
                let mut offset = current_position - *number;
                if offset < LOWER_LIMIT {
                    offset = (offset.abs() + UPPER_LIMIT) / RANGE;
                    println!("PASSED ZERO -> {offset} TIMES");
                    part_two_zero_counter += offset;
                }
                current_position -= *number;
            }
            _ => continue,
        }
        current_position = wrap_around(current_position);
        if current_position == 0 {
            zero_counter += 1;
        }
    }
    println!("PART TWO: {}", part_two_zero_counter + zero_counter);
    zero_counter
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            let instructions = parse_input(&input);
            let answer = get_password(&instructions);
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => eprint!("ERROR: {e}"),
    }
}
