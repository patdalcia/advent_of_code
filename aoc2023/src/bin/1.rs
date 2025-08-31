use aoc2023::get_puzzle;

fn solve_puzzle(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap_or(0);
            let last = digits.next_back().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input);
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
