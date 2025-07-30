use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file() -> Result<File, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_data.txt";
    let file = File::open(file_path)?;
    Ok(file)
}

fn get_do_dont(file: File) -> Result<i32, Box<dyn std::error::Error>> {
    let buf = BufReader::new(file);
    let pattern = Regex::new(r"don't\(\)|do\(\)|mul\(\s*(\d+)\s*,\s*(\d+)\s*\)")?;

    let mut running = true;
    let mut mul_count = 0;
    let mut tail = String::new();

    for line in buf.lines() {
        let line = line?;
        let line_with_tail = tail.clone() + &line;

        for caps in pattern.captures_iter(&line_with_tail) {
            let c = caps.get(0).unwrap().as_str();

            match c {
                "do()" => {
                    running = true;
                }
                "don't()" => {
                    running = false;
                }
                _ => {
                    if running {
                        let a: i32 = caps[1].parse()?;
                        let b: i32 = caps[2].parse()?;
                        mul_count += a * b;
                    }
                }
            }
        }
        tail = if line.len() >= 3 {
            line[line.len() - 3..].to_string()
        } else {
            line.to_string()
        };
    }

    Ok(mul_count)
}

fn main() {
    match get_file() {
        Ok(file) => match get_do_dont(file) {
            Ok(mul_count) => {
                println!("############# ANSWER: {mul_count:#?} ###############");
            }
            Err(e) => {
                println!("ERROR: {e:#?}");
            }
        },
        Err(e) => {
            println!("{e:#?}");
        }
    }
}
