use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

use regex::Regex;

fn get_file() -> Result<File, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;

    Ok(file)
}

fn check_horizontal(lines: &[String], reg: &Regex) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in lines {
        for _ in reg.captures_iter(line) {
            count += 1;
        }
    }
    Ok(count)
}

fn transpose(lines: &[String]) -> Vec<String> {
    if lines.is_empty() {
        return vec![];
    }

    let num_cols = lines[0].len();
    let mut transposed = vec![String::new(); num_cols];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    transposed
}

fn check_vertical(lines: &[String], reg: &Regex) -> Result<i32, Box<dyn std::error::Error>> {
    let columns_as_rows = transpose(lines);

    let mut count = 0;
    for line in columns_as_rows {
        for _ in reg.captures_iter(&line) {
            count += 1;
        }
    }
    Ok(count)
}

fn check_diagonal(lines: &[String], reg: &Regex) -> Result<i32, Box<dyn std::error::Error>> {
    let mut diagonals: Vec<String> = Vec::new();
    let rows = lines.len();
    let cols = lines[0].len();
    let lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let j = 0;
    let mut chars: Vec<char> = Vec::new();
    for i in 0..rows {
        let l = lines[i][j];
        chars.push(l);
    }
    diagonals.push(chars.iter().collect::<String>());

    // TODO: OKAY, we can get diagonal down-right from 0,0. Now we move across row and down column

    println!("DIAGONALS: {diagonals:#?}");
    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reg = Regex::new(r"XMAS|SAMX")?;
    match get_file() {
        Ok(file) => {
            println!("Got the file");
            let buf = BufReader::new(file);
            let lines: Vec<String> = buf.lines().collect::<Result<_, _>>()?;
            match check_horizontal(&lines, &reg) {
                Ok(count) => {
                    println!("HORIZONTAL COUNT: {count:#?}");
                    match check_vertical(&lines, &reg) {
                        Ok(count) => {
                            println!("VERTICAL COUNT: {count:#?}");
                            match check_diagonal(&lines, &reg) {
                                Ok(count) => {
                                    println!("DIAGONAL COUNT: {count:#?}");
                                }
                                Err(e) => {
                                    println!("ERROR: {e:#?}");
                                }
                            }
                        }
                        Err(e) => {
                            println!("CHECK_VERTICAL RETURNED AN ERROR: {e:#?}");
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR: {e:#?}");
                }
            }
        }
        Err(e) => {
            println!("ERROR: {e:#?}");
        }
    }
    Ok(())
}
