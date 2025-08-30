use aoc2022::get_puzzle;
use std::ops::AddAssign;

fn get_round_score(opponent_choice: char, player_choice: char) -> u32 {
    let mut score = 0;
    match player_choice {
        'X' => {
            score.add_assign(1);
            if opponent_choice == 'A' {
                score.add_assign(3);
            } else if opponent_choice == 'C' {
                score.add_assign(6);
            }
        } //Rock
        'Y' => {
            score.add_assign(2);
            if opponent_choice == 'B' {
                score.add_assign(3);
            } else if opponent_choice == 'A' {
                score.add_assign(6);
            }
        } //Paper
        'Z' => {
            score.add_assign(3);
            if opponent_choice == 'C' {
                score.add_assign(3);
            } else if opponent_choice == 'B' {
                score.add_assign(6);
            }
        } //Scissors
        _ => {}
    }
    score
}

fn get_round_score_part_2(opponent_choice: char, player_choice: char) -> u32 {
    let mut score = 0;
    match player_choice {
        'X' => match opponent_choice {
            'A' => {
                score.add_assign(3);
            }
            'B' => {
                score.add_assign(1);
            }
            'C' => {
                score.add_assign(2);
            }
            _ => {}
        }, //Rock
        'Y' => {
            score.add_assign(3);
            match opponent_choice {
                'A' => {
                    score.add_assign(1);
                }
                'B' => {
                    score.add_assign(2);
                }
                'C' => {
                    score.add_assign(3);
                }
                _ => {}
            }
        } //Paper
        'Z' => {
            score.add_assign(6);
            match opponent_choice {
                'A' => {
                    score.add_assign(2);
                }
                'B' => {
                    score.add_assign(3);
                }
                'C' => {
                    score.add_assign(1);
                }
                _ => {}
            }
        } //Scissors
        _ => {}
    }
    score
}

fn solve_puzzle(input: String) -> (u32, u32) {
    let mut answer1 = 0;
    let mut answer2 = 0;
    let _scores: Vec<u32> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[0].chars().count() == 1 && parts[1].chars().count() == 1 {
                let c1 = parts[0].chars().next().unwrap();
                let c2 = parts[1].chars().next().unwrap();
                answer1.add_assign(get_round_score(c1, c2));
                answer2.add_assign(get_round_score_part_2(c1, c2));
                Some(get_round_score(c1, c2))
            } else {
                None
            }
        })
        .collect();
    (answer1, answer2)
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            let answers = solve_puzzle(input);
            println!(
                "ANSWER TO PART ONE: {}\nANSWER TO PART 2: {}",
                answers.0, answers.1
            );
        }
        Err(e) => eprintln!("Error reading file: {e}"),
    }
}
