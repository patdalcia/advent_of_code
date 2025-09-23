use aoc2021::get_puzzle;

// Finds the median of Vec<u32>
fn median(nums: &[u32]) -> u32 {
    let length: u32 = nums.len() as u32;
    let middle_index: u32 = length / 2;

    match nums.len() % 2 {
        0 => {
            let first = nums[(middle_index - 1) as usize];
            let second = nums[(middle_index) as usize];
            (first + second) / 2
        }
        _ => nums[middle_index as usize],
    }
}

fn solve_puzzle1(input: &str) -> u32 {
    let mut horizontal_positions: Vec<u32> = input
        .trim()
        .split(',')
        .filter_map(|num_as_str| num_as_str.parse::<u32>().ok())
        .collect();
    horizontal_positions.sort();
    let med = median(&horizontal_positions);
    horizontal_positions
        .iter()
        .map(|hp| hp.max(&med) - hp.min(&med))
        .sum()
}

fn solve_puzzle2(input: &str) -> u32 {
    let mut scores: Vec<u32> = vec![];
    let mut horizontal_positions: Vec<u32> = input
        .trim()
        .split(',')
        .filter_map(|num_as_str| num_as_str.parse::<u32>().ok())
        .collect();
    horizontal_positions.sort();
    let max_len = horizontal_positions.len() as u32;
    for h in 0..max_len {
        scores.push(
            horizontal_positions
                .iter()
                .map(|hp| {
                    let min = hp.min(&h);
                    let max = hp.max(&h);
                    let mut fuel_cost = 1;
                    let mut fuel_spent = 0;
                    for _ in *min..*max {
                        fuel_spent += fuel_cost;
                        fuel_cost += 1;
                    }
                    fuel_spent
                })
                .sum(),
        );
    }
    scores.sort();
    scores[0]
}

fn main() {
    match get_puzzle("inputs/7.txt") {
        Ok(input) => {
            let answer1 = solve_puzzle1(&input);
            let answer2 = solve_puzzle2(&input);
            println!("ANSWER TO PART ONE: {answer1}");
            println!("ANSWER TO PART TWO: {answer2}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
