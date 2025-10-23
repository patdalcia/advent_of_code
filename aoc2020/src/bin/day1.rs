use aoc2020::get_puzzle;

fn solve_puzzle(input: &str) -> u32 {
    let nums = input
        .trim()
        .lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let nums_clone = nums.clone();
    let mut matched_entries: (u32, u32) = (0, 0);
    'outer: for num in nums {
        for nc in &nums_clone {
            if num + nc == 2020 {
                matched_entries.0 = num;
                matched_entries.1 = *nc;
                break 'outer;
            }
        }
    }
    matched_entries.0 * matched_entries.1
}

fn solve_puzzle2(input: &str) -> u32 {
    let nums = input
        .trim()
        .lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let nums_clone = nums.clone();
    let nums_clone2 = nums.clone();
    let mut matched_entries: (u32, u32, u32) = (0, 0, 0);
    'outer: for num in nums {
        for nc in &nums_clone {
            for nc2 in &nums_clone2 {
                if num + nc + nc2 == 2020 && num != *nc && num != *nc2 && *nc != *nc2 {
                    matched_entries.0 = num;
                    matched_entries.1 = *nc;
                    matched_entries.2 = *nc2;
                    break 'outer;
                }
            }
        }
    }
    println!("{matched_entries:#?}");
    matched_entries.0 * matched_entries.1 * matched_entries.2
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input.as_str());
            let answer2 = solve_puzzle2(input.as_str());
            println!("ANSWER TO PART ONE: {answer}");
            println!("ANSWER TO PART TWO: {answer2}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
