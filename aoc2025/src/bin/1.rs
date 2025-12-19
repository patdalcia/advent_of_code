use aoc2025::get_puzzle;

const STARTING_POINT: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .trim()
        .lines()
        .filter(|line| line.len() > 1)
        .filter_map(|line| {
            let direction = line.chars().next()?;
            let distance = line[1..].parse::<i32>().ok()?;
            Some((direction, distance))
        })
        .collect()
}

fn get_password(instructions: &Vec<(char, i32)>) -> i32 {
    let mut part_one_zero_counter = 0;
    let mut part_two_zero_counter = 0;
    let mut current_position = STARTING_POINT;

    for (direction, distance) in instructions {
        for _ in 0..*distance {
            if *direction == 'R' {
                current_position = (current_position + 1) % DIAL_SIZE;
            } else {
                current_position = (current_position - 1 + DIAL_SIZE) % DIAL_SIZE;
            }

            if current_position == 0 {
                part_two_zero_counter += 1;
            }
        }

        if current_position == 0 {
            part_one_zero_counter += 1;
        }
    }

    println!("PART TWO: {part_two_zero_counter}");
    part_one_zero_counter
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

