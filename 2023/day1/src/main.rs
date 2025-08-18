fn get_num_from_option(option: &&str) -> &'static str {
    match *option {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "0",
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut calibration_value = 0;
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut to_be_tested = String::new();
    let mut first_numbers = vec![];
    let mut last_numbers = vec![];
    for line in input.lines() {
        'chars_forward_loop: for ch in line.chars() {
            if ch.is_numeric() {
                first_numbers.push(ch.to_string());
                break;
            }
            to_be_tested += &ch.to_string();
            for num in &nums {
                if to_be_tested.contains(num) {
                    first_numbers.push(get_num_from_option(num).to_string());
                    to_be_tested.clear();
                    break 'chars_forward_loop;
                }
            }
        }
        to_be_tested.clear();
        'chars_backwards_loop: for ch in line.chars().rev() {
            if ch.is_numeric() {
                last_numbers.push(ch.to_string());
                break;
            }
            to_be_tested = ch.to_string() + &to_be_tested;
            for num in &nums {
                if to_be_tested.contains(num) {
                    last_numbers.push(get_num_from_option(num).to_string());
                    to_be_tested.clear();
                    break 'chars_backwards_loop;
                }
            }
        }
    }

    for (i, first_number) in first_numbers.iter().enumerate() {
        let combined_string = format!("{}{}", first_number, last_numbers[i]);
        let to_be_added: i32 = combined_string.parse().unwrap();
        calibration_value += to_be_added;
    }
    Ok(calibration_value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "ANSWER TO PUZZLE: {}",
        part1(include_str!("puzzle_input.txt"))?
    );
    Ok(())
}
