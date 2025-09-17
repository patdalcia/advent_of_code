use aoc2023::get_puzzle;

fn solve_puzzle(input: String) -> (u32, u32) {
    let mut answer: u32 = 0;
    let mut answer_vec: Vec<u32> = vec![];

    // Solving Part 1
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(_, rest)| rest.split_once('|'))
        .for_each(|(a_str, p_str)| {
            let answer_nums: Vec<u32> = a_str
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            let player_nums: Vec<u32> = p_str
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            let mut temp_score: u32 = 0;

            for a in &answer_nums {
                if player_nums.contains(a) {
                    if temp_score == 0 {
                        temp_score = 1;
                    } else {
                        temp_score = temp_score.saturating_mul(2);
                    }
                }
            }
            answer += temp_score;
            answer_vec.push(temp_score);
        });

    //Solving Part 2
    let mut matches_per_card: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let (winners, draws) = rest.split_once('|').unwrap();

        let winning_nums: Vec<u32> = winners
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let drawn_nums: Vec<u32> = draws
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let matches = winning_nums
            .iter()
            .filter(|n| drawn_nums.contains(n))
            .count() as u32;
        matches_per_card.push(matches);
    }
    let mut counts: Vec<u32> = vec![1u32; matches_per_card.len()];
    for i in 0..matches_per_card.len() {
        let copies = matches_per_card[i];
        let current_count = counts[i];

        for j in (i + 1)..=(i + copies as usize) {
            if j < counts.len() {
                counts[j] += current_count;
            }
        }
    }
    (answer, counts.iter().sum())
}

fn main() {
    match get_puzzle("inputs/3.txt") {
        Ok(input) => {
            let (answer_1, answer_2) = solve_puzzle(input.clone());
            println!("ANSWER PART ONE: {answer_1}\nANSWER PART TWO: {answer_2}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
