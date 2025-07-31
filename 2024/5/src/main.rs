use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn get_lines() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn get_rules(mut lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    // Remove the last line if it's empty
    if let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        }
    }

    let mut rules: Vec<String> = vec![];
    let mut jobs: Vec<String> = vec![];
    let mut empty_index = 0;

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            empty_index = index;
            break;
        }
        rules.push(line.to_string());
    }

    println!("{:#?}", lines[empty_index]);
    jobs = lines[empty_index + 1..].to_vec();

    (rules, jobs)
}

fn main() {
    match get_lines() {
        Ok(lines) => {
            let rules = get_rules(lines.clone());
            println!("{rules:#?}");
        }
        Err(e) => {
            println!("ERROR: {e:#?}");
        }
    }
}
