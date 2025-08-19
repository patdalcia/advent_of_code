use std::collections::HashSet;

fn contains_all(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    for num in b {
        if !(set_a.contains(num)) {
            return false;
        }
    }
    true
}

fn contains_at_all(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    for num in b {
        if set_a.contains(num) {
            return true;
        }
    }
    false
}

fn part_one(input: &str) -> i32 {
    let mut answer = 0;
    for line in input.lines() {
        if let Some((first_half, second_half)) = line.split_once(',') {
            if let (
                Some((first_half_first_number, first_half_second_number)),
                Some((second_half_first_number, second_half_second_number)),
            ) = (first_half.split_once('-'), second_half.split_once('-'))
            {
                let mut first_half_numbers = vec![];
                let mut second_half_numbers = vec![];

                let first_half_start: i32 = first_half_first_number.parse().unwrap();
                let first_half_end: i32 = first_half_second_number.parse().unwrap();

                for i in first_half_start..=first_half_end {
                    first_half_numbers.push(i);
                }
                let second_half_start: i32 = second_half_first_number.parse().unwrap();
                let second_half_end: i32 = second_half_second_number.parse().unwrap();
                for i in second_half_start..=second_half_end {
                    second_half_numbers.push(i);
                }
                if contains_all(&first_half_numbers, &second_half_numbers)
                    || contains_all(&second_half_numbers, &first_half_numbers)
                {
                    answer += 1;
                }
            }
        }
    }
    answer
}

fn part_two(input: &str) -> i32 {
    let mut answer = 0;
    for line in input.lines() {
        if let Some((first_half, second_half)) = line.split_once(',') {
            if let (
                Some((first_half_first_number, first_half_second_number)),
                Some((second_half_first_number, second_half_second_number)),
            ) = (first_half.split_once('-'), second_half.split_once('-'))
            {
                let mut first_half_numbers = vec![];
                let mut second_half_numbers = vec![];

                let first_half_start: i32 = first_half_first_number.parse().unwrap();
                let first_half_end: i32 = first_half_second_number.parse().unwrap();

                for i in first_half_start..=first_half_end {
                    first_half_numbers.push(i);
                }
                let second_half_start: i32 = second_half_first_number.parse().unwrap();
                let second_half_end: i32 = second_half_second_number.parse().unwrap();
                for i in second_half_start..=second_half_end {
                    second_half_numbers.push(i);
                }
                if contains_at_all(&first_half_numbers, &second_half_numbers)
                    || contains_at_all(&second_half_numbers, &first_half_numbers)
                {
                    answer += 1;
                }
            }
        }
    }
    answer
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
