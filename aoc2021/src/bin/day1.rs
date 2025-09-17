use aoc2021::get_puzzle;

fn solve_puzzle(input: String) -> (u32, u32) {
    let mut increased_count_part_1 = 0;
    let mut increased_count_part_2 = 0;
    let parsed_numbers: Vec<u32> = input
        .trim()
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();
    let cloned_parsed_numbers = parsed_numbers.clone();
    let window_sums: Vec<u32> = cloned_parsed_numbers
        .iter()
        .enumerate()
        .map(|(index, current_num)| {
            if let (Some(next_num), Some(next_next_num)) =
                (parsed_numbers.get(index + 1), parsed_numbers.get(index + 2))
            {
                current_num + next_num + next_next_num
            } else {
                0
            }
        })
        .collect();
    let mut num_iter = parsed_numbers.iter().peekable();
    while let Some(current_num) = num_iter.next() {
        if let Some(next_num) = num_iter.peek() {
            if **next_num > *current_num {
                increased_count_part_1 += 1;
            }
        }
    }
    // Part two
    let mut num_iter = window_sums.iter().peekable();
    while let Some(current_window_sum) = num_iter.next() {
        if let Some(next_window_sum) = num_iter.peek() {
            if **next_window_sum > *current_window_sum {
                increased_count_part_2 += 1;
            }
        }
    }
    (increased_count_part_1, increased_count_part_2)
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            let answers = solve_puzzle(input);
            println!(
                "ANSWER TO PART ONE: {}\nANSWER TO PART TWO: {}",
                answers.0, answers.1
            );
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
