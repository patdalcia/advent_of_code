use aoc2022::get_puzzle;

pub fn solve_puzzle(input: String) -> Vec<u32> {
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|group| group.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect();

    totals.sort_unstable_by(|a, b| b.cmp(a));
    totals
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input);
            println!("ANSWER TO PART ONE: {}", answer[0]);
            println!("ANSWER TO PART TWO: {}", answer[0] + answer[1] + answer[2]);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
