use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PuzzleType {
    keys: Vec<i64>,
    values: Vec<Vec<i64>>,
    whitespaces: Vec<usize>,
}

fn count_whitespace_sequences(s: &str) -> usize {
    let re = Regex::new(r"\s+").unwrap();
    re.find_iter(s.trim()).count()
}

fn get_input() -> Result<PuzzleType, Box<dyn Error>> {
    let file_path = "data/puzzle_input_test.txt";
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    let mut keys: Vec<i64> = vec![];
    let mut values: Vec<Vec<i64>> = vec![];
    let mut whitespaces: Vec<usize> = vec![];

    for line in buf.lines() {
        let line = line?;
        let trimmed = line.trim();

        let (key, value) = trimmed
            .split_once(':')
            .ok_or("ERROR: Could not split string in get_input")?;

        // ✅ parse to i64 now
        keys.push(key.trim().parse::<i64>()?);

        whitespaces.push(count_whitespace_sequences(value));

        let value_parts: Result<Vec<i64>, _> = value
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect();

        values.push(value_parts?);
    }

    Ok(PuzzleType {
        keys,
        values,
        whitespaces,
    })
}

fn generate_binary_combinations(n: usize) -> Vec<Vec<u8>> {
    let choices: [u8; 3] = [0, 1, 2]; // 0, 1, and symbolic "11"
    let mut result = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        n: usize,
        index: usize,
        current: &mut Vec<u8>,
        result: &mut Vec<Vec<u8>>,
        choices: &[u8],
    ) {
        if index == n {
            result.push(current.clone());
            return;
        }

        for &choice in choices {
            current.push(choice);
            backtrack(n, index + 1, current, result, choices);
            current.pop();
        }
    }

    backtrack(n, 0, &mut current, &mut result, &choices);
    result
}

fn is_holder_complete(holder: &(Option<i64>, Option<u8>, Option<i64>)) -> bool {
    holder.0.is_some() && holder.1.is_some() && holder.2.is_some()
}

fn concatenate(a: &i64, b: &i64) -> Result<i64, Box<dyn std::error::Error>> {
    let concatenated = format!("{a}{b}");
    Ok(concatenated.parse::<i64>()?)
}

#[allow(clippy::get_first)]
fn solve_puzzle(puzzle: PuzzleType) -> Result<i64, Box<dyn std::error::Error>> {
    let mut answer = 0i64;

    for (index, value) in puzzle.values.iter().enumerate() {
        let combinations = generate_binary_combinations(puzzle.whitespaces[index]);

        for mut combo in combinations {
            let mut i = 0;
            let mut left_side_argument: i64 = 0;
            let mut right_side_argument: i64 = 0;
            let mut operator: u8 = 0;

            while i + 1 < value.len() {
                if left_side_argument == 0 {
                    left_side_argument = value[i];
                }

                right_side_argument = value[i + 1];

                if let Some(op) = combo.get(0) {
                    operator = *op;
                } else {
                    break;
                }
                combo.remove(0);

                let temp = match operator {
                    0 => left_side_argument + right_side_argument,
                    1 => left_side_argument * right_side_argument,
                    2 => concatenate(&left_side_argument, &right_side_argument)?,
                    _ => {
                        println!("ERROR: Invalid Operator inside of solve_puzzle");
                        break;
                    }
                };

                left_side_argument = temp;
                i += 1;
            }

            if left_side_argument == puzzle.keys[index] {
                answer += puzzle.keys[index];
                break;
            }
        }
    }

    Ok(answer)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input: PuzzleType = get_input()?;
    let valid_counter = solve_puzzle(puzzle_input)?;
    println!("FINAL VALID COUNTER: {valid_counter:#?}");
    Ok(())
}
