use aoc2025::get_puzzle;
use std::cmp;

// A nice little algorithm
fn merge_overlapping_ranges(arr: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    arr.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result: Vec<(u64, u64)> = Vec::new();
    result.push(arr[0]);

    for i in 1..arr.len() {
        let current: (u64, u64) = arr[i];
        let j: usize = result.len() - 1;

        if current.0 >= result[j].0 && current.0 <= result[j].1 {
            result[j].1 = cmp::max(current.1, result[j].1);
        } else if current.1 >= result[j].0 && current.1 <= result[j].1 {
            result[j].0 = cmp::min(current.0, result[j].0);
        } else if !result.contains(&current) {
            result.push(current);
        }
    }
    result
}

fn solve_puzzle(input: &str) -> (u64, u64) {
    println!("~~INPUT~~\n{input}");
    let mut fresh_count = 0;
    let (before, after) = input
        .trim()
        .split_once("\n\n")
        .expect("COULD NOT SPLIT INPUT AT BLANK LINE");
    let mut ranges: Vec<(u64, u64)> = before
        .lines()
        .map(|line| line.split_once('-').expect("COULD NOT SPLIT LINE ON -"))
        .filter_map(|(start_str, end_str)| {
            match (
                start_str.trim().parse::<u64>(),
                end_str.trim().parse::<u64>(),
            ) {
                (Ok(start), Ok(end)) => Some((start, end)),
                _ => None,
            }
        })
        .collect();

    let condensed_ranges = merge_overlapping_ranges(&mut ranges);

    let ingredient_ids: Vec<u64> = after
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    for id in ingredient_ids {
        for (start, end) in &condensed_ranges {
            let range = start..=end;
            if range.contains(&&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    let mut answer_part_2 = 0;
    for range in &condensed_ranges {
        answer_part_2 += range.1.abs_diff(range.0) + 1;
    }

    (fresh_count, answer_part_2)
}

fn main() {
    match get_puzzle("inputs/5.txt") {
        Ok(input) => {
            let (answer_1, answer_2) = solve_puzzle(&input);
            println!("ANSWER TO PART ONE: {answer_1}\nANSWER TO PART TWO: {answer_2}");
        }
        Err(e) => {
            eprintln!("ERROR IN MAIN: {e}");
        }
    }
}
