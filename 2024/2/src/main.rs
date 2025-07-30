use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_safe(parts: Vec<&str>) -> Result<bool, Box<dyn std::error::Error>> {
    let numbers: Vec<i32> = parts.iter().map(|p| p.parse()).collect::<Result<_, _>>()?;

    if numbers.len() < 2 {
        return Ok(false);
    }

    let mut increasing: Option<bool> = None;

    for pair in numbers.windows(2) {
        let a = pair[0];
        let b = pair[1];

        if a == b {
            return Ok(false);
        }

        if increasing.is_none() {
            if b > a && b <= a.saturating_add(3) {
                increasing = Some(true);
                continue;
            } else if b < a && b >= a.saturating_sub(3) {
                increasing = Some(false);
                continue;
            } else {
                return Ok(false);
            }
        }

        match increasing {
            Some(true) => {
                if b <= a || b > a.saturating_add(3) {
                    return Ok(false);
                }
            }
            Some(false) => {
                if b >= a || b < a.saturating_sub(3) {
                    return Ok(false);
                }
            }
            None => unreachable!(),
        }
    }

    Ok(true)
}

fn remove_one_and_check(parts: Vec<&str>) -> Result<bool, Box<dyn std::error::Error>> {
    for i in 0..parts.len() {
        let mut modified = parts.clone();
        modified.remove(i);
        let result = check_safe(modified)?;
        if result {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_data() -> Result<i32, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);

    let mut safe_count = 0;
    for line in buf.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split_whitespace().collect();
        match check_safe(parts.clone()) {
            Ok(is_safe) => {
                if is_safe {
                    safe_count += 1;
                } else {
                    match remove_one_and_check(parts) {
                        Ok(value) => {
                            if value {
                                safe_count += 1
                            }
                        }
                        Err(e) => {
                            println!("ERROR: {e:#?}");
                        }
                    }
                }
            }
            Err(e) => {
                println!("ERROR: {e:?}");
            }
        }
    }

    Ok(safe_count)
}

fn main() {
    match get_data() {
        Ok(value) => {
            println!("{value:#?}");
        }
        Err(e) => {
            println!("ERROR: {e:#?}");
        }
    }
}
