use aoc2021::get_puzzle;

struct Point {
    x: i32,
    y: i32,
}

fn solve_puzzle(input: &str) {
    println!("{input}");
}

fn main() {
    match get_puzzle("inputs/17.txt") {
        Ok(input) => {
            solve_puzzle(input.as_str());
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
