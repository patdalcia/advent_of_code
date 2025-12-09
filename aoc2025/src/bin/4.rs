use aoc2025::get_puzzle;

fn solve_puzzle(input: &str) {
    println!("~~ INPUT ~~\n{input}");
    let line_len = input.lines().next().unwrap().len();
    let grid: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    println!("{line_len}");
}

fn main() {
    match get_puzzle("inputs/4.txt") {
        Ok(input) => {
            solve_puzzle(&input);
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}
