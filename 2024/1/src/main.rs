use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Pulls information from file, splits whitespace, sorts and saves to two vectors
fn get_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    let mut first_column_numbers: Vec<i32> = Vec::new();
    let mut second_column_numbers: Vec<i32> = Vec::new();
    for line_result in buf.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() {
            first_column_numbers.push(parts[0].parse()?);
            second_column_numbers.push(parts[1].parse()?);
        }
    }
    first_column_numbers.sort();
    second_column_numbers.sort();

    //let count = values.iter().filter(|x| *x == target).count();

    Ok((first_column_numbers, second_column_numbers))
}

fn compare_distance(first_column_numbers: Vec<i32>, second_column_numbers: Vec<i32>) {
    let mut distance: i32 = 0;
    let mut similarity_score: i32 = 0;
    for i in 0..first_column_numbers.len() {
        let first_multiplier = second_column_numbers
            .iter()
            .filter(|&&x| x == first_column_numbers[i])
            .count() as i32;
        similarity_score += first_column_numbers[i] * first_multiplier;

        distance += max(first_column_numbers[i], second_column_numbers[i])
            - min(first_column_numbers[i], second_column_numbers[i]);
    }
    println!("#################### DISTANCE: {distance} ###################");
    println!("#################### SIMILARITY: {similarity_score} ###################");
}
//<BS>let result = max(a, b) - min(a, b);
fn main() {
    match get_input() {
        Ok((first_col, second_col)) => {
            compare_distance(first_col, second_col);
        }
        Err(e) => {
            println!("ERROR {e:?}");
        }
    }
}
