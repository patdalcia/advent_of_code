use aoc2020::get_puzzle;

fn validate_data(input: (&str, &str)) -> bool {
    let data = input.1;
    match input.0 {
        "byr" => {
            if data.len() == 4 {
                if let Ok(data_parsed) = data.parse::<u32>() {
                    if (1920..=2002).contains(&data_parsed) {
                        return true;
                    }
                }
            }
            false
        }
        "iyr" => {
            if data.len() == 4 {
                if let Ok(data_parsed) = data.parse::<u32>() {
                    if (2010..=2020).contains(&data_parsed) {
                        return true;
                    }
                }
            }
            false
        }
        "eyr" => {
            if data.len() == 4 {
                if let Ok(data_parsed) = data.parse::<u32>() {
                    if (2020..=2030).contains(&data_parsed) {
                        return true;
                    }
                }
            }
            false
        }
        "hgt" => {
            if let Some(rest) = data.strip_suffix("in") {
                if let Ok(parsed_data) = rest.parse::<u32>() {
                    if (59..=76).contains(&parsed_data) {
                        return true;
                    }
                }
            } else if let Some(rest) = data.strip_suffix("cm") {
                if let Ok(parsed_data) = rest.parse::<u32>() {
                    if (150..=193).contains(&parsed_data) {
                        return true;
                    }
                }
            }
            false
        }
        "hcl" => {
            if data.len() == 7 {
                let mut chars = data.chars();
                if let Some(first_char) = chars.next()
                    && first_char == '#'
                {
                    for ch in chars {
                        if !ch.is_digit(36) {
                            return false;
                        }
                    }
                    return true;
                }
            }
            false
        }
        "ecl" => {
            data == "amb"
                || data == "blu"
                || data == "brn"
                || data == "gry"
                || data == "grn"
                || data == "hzl"
                || data == "oth"
        }
        "pid" => {
            if data.len() == 9 {
                if let Ok(_data_parsed) = data.parse::<u32>() {
                    return true;
                }
            }
            false
        }
        "cid" => {
            //
            true
        }
        _ => {
            //
            false
        }
    }
}

fn solve_puzzle(input: &str) -> u32 {
    let mut valid_field_count = 0;
    let mut field_checker: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        "cid".to_string(),
    ];
    let mut field_checker_2: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    ];
    field_checker.sort();
    field_checker_2.sort();
    let mut passport_fields: Vec<(String, String)> = Vec::new();
    let mut valid_count = 0;
    'outer: for line in input.lines() {
        if line.is_empty() {
            // parse passport
            let mut temp_passport = Vec::new();
            for p in &passport_fields {
                temp_passport.push(String::from(&p.0));
            }
            temp_passport.sort();
            if temp_passport == field_checker || temp_passport == field_checker_2 {
                for field in &passport_fields {
                    if !validate_data((field.0.as_str(), field.1.as_str())) {
                        continue 'outer;
                    }
                }
                valid_count += 1;
                print!("VALID -> ");
                for test in &passport_fields {
                    print!("{} ", test.0);
                }
                println!(" ");
            }
            passport_fields.clear();
            continue;
        }
        for sw in line.split_whitespace() {
            if let Some(passport_field) = sw.split_once(':') {
                passport_fields.push((
                    String::from(passport_field.0),
                    String::from(passport_field.1),
                ));
            }
        }
    }
    valid_count
}

fn main() {
    match get_puzzle("inputs/4.txt") {
        Ok(input) => {
            let answer = solve_puzzle(&input);
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}

