use aoc2024::get_puzzle;

fn solve_puzzle(input: String) {
    let mut flag = true;
    input.lines().for_each(|line| {
        line.split_whitespace().for_each(|num_as_str| {
            if let Ok(num) = num_as_str.parse::<u32>() {
                println!("num: {num}");
            }
        });
    });
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
