use aoc2022::get_puzzle;

fn solve_puzzle(input: String) {
    let pairs_as_str: Vec<(&str, &str)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    for pair_as_string in pairs_as_str {
        let first_nums: Vec<i32> = dbg!(
            pair_as_string
                .0
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect()
        );
    }
}

fn main() {
    match get_puzzle("inputs/13.txt") {
        Ok(input) => {
            solve_puzzle(input);
        }
        Err(e) => {
            eprintln!("ERROR: {e}")
        }
    }
}
