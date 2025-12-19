use aoc2025::get_puzzle;
use indicatif::ProgressIterator;

fn is_entirely_repeating(input: &str, curr_pattern: &str) -> bool {
    if curr_pattern.is_empty() || input.is_empty() {
        return false;
    }
    // Check if lengths are compatible and if input consists only of the pattern
    input.len() % curr_pattern.len() == 0 && input.replace(curr_pattern, "").is_empty()
}

fn part_one(input: &str) -> i64 {
    println!("~~INPUT~~\n{input}");
    let mut answer_1 = 0;

    let ranges: Vec<&str> = input.trim().split(',').collect();

    for range in ranges {
        if range.starts_with('0') {
            continue;
        }
        if let Some((s_str, e_str)) = range.split_once('-') {
            let start = s_str.trim().parse::<u64>();
            let end = e_str.trim().parse::<u64>();

            if let (Ok(s), Ok(e)) = (start, end) {
                for num in s..=e {
                    let num_as_str = num.to_string();
                    let len = num_as_str.len();

                    if len > 0 && len % 2 == 0 {
                        let mid = len / 2;
                        let (first, second) = num_as_str.split_at(mid);
                        if first == second {
                            let whole_repeated_number = format!("{first}{second}");
                            if let Ok(parsed_repeat) = whole_repeated_number.parse::<i64>() {
                                answer_1 += parsed_repeat;
                            }
                        }
                    }
                }
            }
        }
    }
    answer_1
}

fn part_two(input: &str) -> i64 {
    println!("~~ INPUT ~~\n{input}");
    let mut answer2 = 0;
    let ranges: Vec<&str> = input.trim().split(',').collect();
    for range in ranges.iter().progress() {
        if range.starts_with('0') {
            continue;
        }
        if let Some((s_str, e_str)) = range.split_once('-') {
            let start = s_str.trim().parse::<u64>().unwrap_or(0);
            let end = e_str.trim().parse::<u64>().unwrap_or(0);
            for num in start..=end {
                let num_as_str = num.to_string();
                let chars: Vec<char> = num_as_str.chars().collect();
                let len = chars.len();
                if len > 0 && len % 2 == 0 {
                    let mid = len / 2;
                    let (first, second) = num_as_str.split_at(mid);
                    if first == second {
                        let whole_repeated_number = format!("{first}{second}");
                        if let Ok(parsed_repeat) = whole_repeated_number.parse::<i64>() {
                            answer2 += parsed_repeat;
                            continue;
                        }
                    }
                }
                if len > 0 {
                    for i in 1..=(len / 2) {
                        let curr_pattern = &chars[0..i].iter().collect::<String>();
                        if i + i <= len && is_entirely_repeating(&num_as_str, curr_pattern) {
                            if let Ok(parsed_repeat) = num_as_str.parse::<i64>() {
                                answer2 += parsed_repeat;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    answer2
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            let answer_1 = part_one(&input);
            let answer_2 = part_two(&input);
            println!("ANSWER TO PART ONE: {answer_1}\nANSWER TO PART TWO: {answer_2}");
        }
        Err(e) => {
            eprintln!("ERROR IN MAIN: {e}");
        }
    }
}
