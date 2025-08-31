use aoc2023::get_puzzle;
use std::ops::AddAssign;

struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

fn solve_puzzle(input: String) -> (u32, u32) {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let mut answer_part_one = 0;
    let mut answer_part_two = 0;

    input
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| line.split_once(':').map(|(_, rest)| (line_index, rest)))
        .for_each(|(line_index, split_once_line)| {
            let mut color_watcher_clone = Colors {
                red: 0,
                green: 0,
                blue: 0,
            };

            let mut flag = true;
            split_once_line.split(';').for_each(|hand| {
                let mut counts = Colors {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                hand.split_whitespace()
                    .map(|part| part.trim_matches(|c| [',', ';', ':'].contains(&c)))
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .for_each(|pair| {
                        if let [num_str, color_str] = pair {
                            if let Ok(num) = num_str.parse::<u32>() {
                                match *color_str {
                                    "red" => counts.red += num,
                                    "green" => counts.green += num,
                                    "blue" => counts.blue += num,
                                    _ => println!("Unexpected color: {color_str}"),
                                }
                            }
                        }
                    });

                color_watcher_clone.red = color_watcher_clone.red.max(counts.red);
                color_watcher_clone.green = color_watcher_clone.green.max(counts.green);
                color_watcher_clone.blue = color_watcher_clone.blue.max(counts.blue);

                if flag {
                    flag = counts.red <= RED_MAX
                        && counts.green <= GREEN_MAX
                        && counts.blue <= BLUE_MAX;
                }
            });

            if flag {
                answer_part_one.add_assign(line_index as u32 + 1);
            }
            answer_part_two.add_assign(
                color_watcher_clone.red * color_watcher_clone.green * color_watcher_clone.blue,
            );
        });

    (answer_part_one, answer_part_two)
}

fn main() {
    match get_puzzle("inputs/2.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input);
            println!("ANSWER TO PART ONE: {}", answer.0);
            println!("ANSWER TO PART TWO: {}", answer.1);
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
