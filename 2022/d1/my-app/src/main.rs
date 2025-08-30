use my_lib::solve_puzzle;

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let answer = solve_puzzle(input);
    println!("ANSWER TO PART ONE: {}", answer[0]);
    println!("ANSWER TO PART TWO: {}", answer[0] + answer[1] + answer[2]);
}
