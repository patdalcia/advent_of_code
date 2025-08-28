use std::ops::AddAssign;

fn solve_puzzle(input: &str) -> Vec<u32> {
    let mut temp_total = 0;
    let mut totals = vec![];
    for line in input.lines() {
        if let Ok(num) = line.parse::<u32>() {
            temp_total.add_assign(num);
        } else {
            totals.push(temp_total);
            temp_total = 0;
        }
    }
    totals.push(temp_total);
    totals.sort_by(|a, b| b.cmp(a));
    totals
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let totals = solve_puzzle(input);
    println!("ANSWER TO PART ONE: {}", totals[0]);
    println!("ANSWER TO PART TWO: {}", totals[0] + totals[1] + totals[2]);
}
