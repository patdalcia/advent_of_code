use aoc2025::get_puzzle;

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
                            println!("REPEAT FOUND: {whole_repeated_number}");
                        }
                    }
                }
            }
        }
    }
    answer_1
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            let answer_1 = part_one(&input);
            println!("ANSWER TO PART ONE: {answer_1}");
        }
        Err(e) => {
            eprintln!("ERROR IN MAIN: {e}");
        }
    }
}
