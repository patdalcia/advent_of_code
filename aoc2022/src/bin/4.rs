use aoc2022::get_puzzle;
use std::{collections::HashSet, ops::AddAssign};

fn contains_all(a: &[u32], b: &[u32]) -> bool {
    let set_a: HashSet<u32> = a.iter().copied().collect();
    for num in b {
        if !(set_a.contains(num)) {
            return false;
        }
    }
    true
}
fn contains_at_all(a: &[u32], b: &[u32]) -> bool {
    let set_a: HashSet<u32> = a.iter().copied().collect();
    for num in b {
        if set_a.contains(num) {
            return true;
        }
    }
    false
}

fn solve_puzzle(input: String) -> (u32, u32) {
    let mut answer_part_one = 0;
    let mut answer_part_two = 0;
    input.lines().for_each(|line| {
        if let Some((first, second)) = line.split_once(',') {
            if let (
                Some((left_first_num, left_second_num)),
                Some((right_first_num, right_second_num)),
            ) = (first.split_once('-'), second.split_once('-'))
            {
                if let (Ok(l_first_num), Ok(l_second_num), Ok(r_first_num), Ok(r_second_num)) = (
                    left_first_num.parse::<u32>(),
                    left_second_num.parse::<u32>(),
                    right_first_num.parse::<u32>(),
                    right_second_num.parse::<u32>(),
                ) {
                    let first_range: Vec<u32> = (l_first_num..=l_second_num).collect();
                    let second_range: Vec<u32> = (r_first_num..=r_second_num).collect();
                    if contains_all(&first_range, &second_range)
                        || contains_all(&second_range, &first_range)
                    {
                        answer_part_one.add_assign(1);
                    }
                    if contains_at_all(&first_range, &second_range)
                        || contains_at_all(&second_range, &first_range)
                    {
                        answer_part_two.add_assign(1);
                    }
                }
            }
        }
    });
    (answer_part_one, answer_part_two)
}

fn main() {
    match get_puzzle("inputs/4.txt") {
        Ok(input) => {
            let (answer_part_one, answer_part_two) = solve_puzzle(input);
            println!("ANSWER TO PART ONE: {answer_part_one}");
            println!("ANSWER TO PART TWO: {answer_part_two}");
        }
        Err(e) => {
            println!("ERROR: Reading File: {e}")
        }
    }
}
