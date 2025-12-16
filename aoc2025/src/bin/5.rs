use aoc2025::get_puzzle;
use indicatif::ProgressIterator;
use std::{thread::sleep, time::Duration};

fn solve_puzzle(input: &str) -> (u64, usize) {
    println!("~~INPUT~~\n{input}");
    let mut fresh_count = 0;
    let mut available_fresh_ids: Vec<u64> = Vec::new();
    let (before, after) = input
        .trim()
        .split_once("\n\n")
        .expect("COULD NOT SPLIT INPUT AT BLANK LINE");
    let ranges: Vec<(u64, u64)> = before
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

    let ingredient_ids: Vec<u64> = after
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();
    println!("Parsed Ranges: {ranges:?}");
    println!("Parsed Ingredient Id's: {ingredient_ids:?}");

    for id in ingredient_ids {
        for (start, end) in &ranges {
            let range = start..=end;
            if range.contains(&&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    for (start, end) in ranges.iter().progress() {
        for id in *start..=*end {
            if !available_fresh_ids.contains(&id) {
                available_fresh_ids.push(id);
            }
        }
    }
    (fresh_count, available_fresh_ids.len())
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
