use aoc2021::get_puzzle;

fn parse_binary_number(input: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let intval = isize::from_str_radix(input, 2)?;
    Ok(intval)
}

fn solve_puzzle1(input: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let binary_as_chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut binaries: Vec<Vec<u32>> = vec![];
    for row in binary_as_chars {
        binaries.push(row.iter().filter_map(|ch| ch.to_digit(10)).collect());
    }
    let row_limit = binaries[0].len();
    let mut nums_from_columns: Vec<Vec<u32>> = vec![];
    for col in 0..row_limit {
        let mut selected_nums: Vec<u32> = vec![];
        for row in &binaries {
            if let Some(num) = row.get(col) {
                selected_nums.push(*num);
            }
        }
        nums_from_columns.push(selected_nums);
    }
    let mut gamma_binaries = vec![];
    let mut epsilon_binaries = vec![];
    for row in &nums_from_columns {
        let one_count = row.iter().filter(|num| **num == 1).count();
        let zero_count = row.iter().filter(|num| **num == 0).count();
        if one_count > zero_count {
            gamma_binaries.push(1);
            epsilon_binaries.push(0);
        } else {
            gamma_binaries.push(0);
            epsilon_binaries.push(1);
        }
    }
    let gamma_as_string: String = gamma_binaries.iter().map(|line| line.to_string()).collect();
    let epsilon_as_string: String = epsilon_binaries
        .iter()
        .map(|line| line.to_string())
        .collect();
    let gamma = parse_binary_number(&gamma_as_string)?;
    let epsilon = parse_binary_number(&epsilon_as_string)?;

    Ok(gamma * epsilon)
}

fn solve_puzzle2(input: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let binary_as_chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let binaries: Vec<Vec<u32>> = binary_as_chars
        .into_iter()
        .map(|row| row.iter().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let row_limit = binaries[0].len();

    let mut nums_from_columns: Vec<Vec<u32>> = Vec::with_capacity(row_limit);
    for col in 0..row_limit {
        let mut selected_nums = Vec::with_capacity(binaries.len());
        for row in &binaries {
            if let Some(&num) = row.get(col) {
                selected_nums.push(num);
            }
        }
        nums_from_columns.push(selected_nums);
    }

    let mut oxygen_generator_rating = binaries.clone();
    let mut co2_scrubber_rating = binaries.clone();

    for index in 0..row_limit {
        if oxygen_generator_rating.len() == 1 {
            break;
        }

        let (one_count, zero_count) = {
            let mut o = 0;
            let mut z = 0;
            for row in &oxygen_generator_rating {
                if let Some(&b) = row.get(index) {
                    if b == 1 {
                        o += 1;
                    } else if b == 0 {
                        z += 1;
                    }
                }
            }
            (o, z)
        };

        let keep_bit = if one_count >= zero_count { 1 } else { 0 };

        oxygen_generator_rating.retain(|row| {
            if let Some(&b) = row.get(index) {
                b == keep_bit
            } else {
                false
            }
        });
    }

    for index in 0..row_limit {
        if co2_scrubber_rating.len() == 1 {
            break;
        }

        let (one_count, zero_count) = {
            let mut o = 0;
            let mut z = 0;
            for row in &co2_scrubber_rating {
                if let Some(&b) = row.get(index) {
                    if b == 1 {
                        o += 1;
                    } else if b == 0 {
                        z += 1;
                    }
                }
            }
            (o, z)
        };

        let keep_bit = if zero_count <= one_count { 0 } else { 1 };

        co2_scrubber_rating.retain(|row| {
            if let Some(&b) = row.get(index) {
                b == keep_bit
            } else {
                false
            }
        });
    }

    let o = parse_binary_number(
        oxygen_generator_rating[0]
            .iter()
            .map(|line| line.to_string())
            .collect::<String>()
            .as_str(),
    )?;

    let c = parse_binary_number(
        co2_scrubber_rating[0]
            .iter()
            .map(|line| line.to_string())
            .collect::<String>()
            .as_str(),
    )?;

    Ok(o * c)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match get_puzzle("inputs/3.txt") {
        Ok(input) => {
            let answer1 = solve_puzzle1(input.as_str())?;
            let answer2 = solve_puzzle2(input.as_str())?;
            println!("ANSWER TO PART ONE: {answer1}");
            println!("ANSWER TO PART TWO: {answer2}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
    Ok(())
}
