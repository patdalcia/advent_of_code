use aoc2024::get_puzzle;

fn solve_puzzle(input: String) {
    println!("{input}");
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            solve_puzzle(input);
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
