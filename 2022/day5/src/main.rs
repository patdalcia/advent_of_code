use std::vec;

fn part_one(input: &str) -> String {
    let mut crates_stripped: Vec<Vec<char>> = vec![];

    if let Some((crates, instructions)) = input.split_once("\n\n") {
        for line in crates.lines().rev().skip(1) {
            for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk.len() > 1 && chunk[1].is_ascii_alphabetic() {
                    let ch = chunk[1] as char;
                    if crates_stripped.len() <= i {
                        crates_stripped.resize_with(i + 1, Vec::new);
                    }
                    crates_stripped[i].push(ch);
                }
            }
        }

        for line in instructions.lines() {
            let instruction_numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect();

            if instruction_numbers.len() == 3 {
                let (move_count, move_from, move_to) = (
                    instruction_numbers[0],
                    instruction_numbers[1] - 1,
                    instruction_numbers[2] - 1,
                );

                for _ in 0..move_count {
                    if let Some(to_be_moved) = crates_stripped[move_from].pop() {
                        crates_stripped[move_to].push(to_be_moved);
                    }
                }
            }
        }
    }

    let mut answer = String::new();
    for mut line in crates_stripped {
        answer.push(line.pop().unwrap());
    }
    answer
}

fn part_two(input: &str) -> String {
    let mut crates_stripped: Vec<Vec<char>> = vec![];

    if let Some((crates, instructions)) = input.split_once("\n\n") {
        for line in crates.lines().rev().skip(1) {
            for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk.len() > 1 && chunk[1].is_ascii_alphabetic() {
                    let ch = chunk[1] as char;
                    if crates_stripped.len() <= i {
                        crates_stripped.resize_with(i + 1, Vec::new);
                    }
                    crates_stripped[i].push(ch);
                }
            }
        }

        for line in instructions.lines() {
            let instruction_numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect();

            if instruction_numbers.len() == 3 {
                let (move_count, move_from, move_to) = (
                    instruction_numbers[0],
                    instruction_numbers[1] - 1,
                    instruction_numbers[2] - 1,
                );
                let mut temp_crates_stripped = vec![];
                for _ in 0..move_count {
                    if let Some(to_be_moved) = crates_stripped[move_from].pop() {
                        temp_crates_stripped.insert(0, to_be_moved);
                    }
                }
                for to_be_moved in temp_crates_stripped {
                    crates_stripped[move_to].push(to_be_moved);
                }
            }
        }
    }

    let mut answer = String::new();
    for mut line in crates_stripped {
        answer.push(line.pop().unwrap());
    }
    answer
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
