use aoc2020::get_puzzle;

fn solve_puzzle(input: &str) {
    println!("{input}");
    let split_lines: Vec<(&str, &str)> = input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(':'))
        .collect();
    for sl in split_lines {
        let policy_split = sl.0.split_once(' ').unwrap();
        let policy_range_not_parsed = policy_split.0.split_once('-').unwrap();
        let policy_char = policy_split.1.trim().chars();
        let pr1 = policy_range_not_parsed.0.parse::<u32>().unwrap();
        let pr2 = policy_range_not_parsed.1.parse::<u32>().unwrap();
    }
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            solve_puzzle(input.as_str());
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
