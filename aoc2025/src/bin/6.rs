use aoc2025::get_puzzle;

fn solve_puzzle(mut input: &str) -> (u64, u64) {
    println!("~~ Input ~~\n{input}");
    input = input.trim();

    let line_width = input
        .lines()
        .next()
        .expect("ERROR: Could not get line length from input -> in SOLVE_PUZZLE")
        .split_whitespace()
        .count();

    let operators: Vec<char> = input
        .lines()
        .last()
        .expect("ERROR: Could not get OPERATORS from input -> in SOLVE_PUZZLE")
        .split_whitespace()
        .filter_map(|split_line| split_line.trim().chars().next())
        .collect();

    let number_grid: Vec<Vec<u64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|split_line| split_line.parse::<u64>().ok())
                .collect()
        })
        .collect();

    // Sorting Grid For Part One
    let mut sorted_grid: Vec<Vec<u64>> = Vec::new();
    for x in 0..line_width {
        let mut temp_vec: Vec<u64> = Vec::new();
        for line in &number_grid {
            if let Some(current_num) = line.get(x) {
                temp_vec.push(*current_num);
            }
        }

        sorted_grid.push(temp_vec);
    }

    let mut answer_part_one: u64 = 0;
    let mut answer_part_two: u64 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let line_width = lines[0].len();

    // Collect columns as strings for part two
    let mut columns: Vec<String> = vec![String::new(); line_width];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    let mut columns_as_nums: Vec<Vec<u64>> = Vec::new();
    let mut temp_line_of_nums: Vec<u64> = Vec::new();
    for column in &columns {
        let column_trimmed = column.trim();
        let num_str: String = column_trimmed
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        if !num_str.is_empty() {
            temp_line_of_nums.push(
                num_str
                    .parse::<u64>()
                    .expect("ERROR: Could not parse number for PART TWO"),
            );
        } else {
            columns_as_nums.push(temp_line_of_nums.clone());
            temp_line_of_nums.clear();
        }
    }
    columns_as_nums.push(temp_line_of_nums.clone());
    temp_line_of_nums.clear();

    for (index, operator) in operators.iter().enumerate() {
        if let Some(current_line) = sorted_grid.get(index)
            && let Some(current_line_part_two) = columns_as_nums.get(index)
        {
            match operator {
                '*' => {
                    answer_part_one += current_line.iter().product::<u64>();
                    answer_part_two += current_line_part_two.iter().product::<u64>();
                }
                '+' => {
                    answer_part_one += current_line.iter().sum::<u64>();
                    answer_part_two += current_line_part_two.iter().sum::<u64>();
                }
                _ => {
                    //
                }
            }
        }
    }

    (answer_part_one, answer_part_two)
}

fn main() {
    match get_puzzle("inputs/6.txt") {
        Ok(input) => {
            let answers = solve_puzzle(&input);
            println!(
                "~~ANSWERS~~\n--- Part One -> {}\n--- Part Two -> {}",
                answers.0, answers.1
            );
        }
        Err(e) => {
            eprint!("ERROR: {e}");
        }
    }
}
