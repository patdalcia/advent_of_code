fn part_one(input: &str) -> i32 {
    let input_as_array = input.lines();
    let mut total_per_elf = vec![];
    let mut temp_totals = vec![];
    for line in input_as_array {
        if line.is_empty() {
            let mut temp_total = 0;
            temp_totals.iter().for_each(|temp| {
                temp_total += temp;
            });
            total_per_elf.push(temp_total);
            temp_totals.clear();
        } else if let Ok(number) = line.parse::<i32>() {
            temp_totals.push(number);
        }
    }
    if !(temp_totals.is_empty()) {
        let mut temp_total = 0;
        temp_totals.iter().for_each(|temp| {
            temp_total += temp;
        });
        total_per_elf.push(temp_total);
        temp_totals.clear();
    }
    let mut highest_calorie_count = 0;
    for total in total_per_elf {
        if total > highest_calorie_count {
            highest_calorie_count = total;
        }
    }
    highest_calorie_count
}
fn part_two(input: &str) -> i32 {
    let input_as_array = input.lines();
    let mut total_per_elf = vec![];
    let mut temp_totals = vec![];
    for line in input_as_array {
        if line.is_empty() {
            let mut temp_total = 0;
            temp_totals.iter().for_each(|temp| {
                temp_total += temp;
            });
            total_per_elf.push(temp_total);
            temp_totals.clear();
            continue;
        }
        if let Ok(number) = line.parse::<i32>() {
            temp_totals.push(number);
        }
    }
    if !(temp_totals.is_empty()) {
        let mut temp_total = 0;
        temp_totals.iter().for_each(|temp| {
            temp_total += temp;
        });
        total_per_elf.push(temp_total);
        temp_totals.clear();
    }
    let mut highest_calorie_count = 0;
    let mut second_highest_calorie_count = 0;
    let mut third_highest_calorie_count = 0;
    for total in total_per_elf {
        if total > highest_calorie_count {
            third_highest_calorie_count = second_highest_calorie_count;
            second_highest_calorie_count = highest_calorie_count;
            highest_calorie_count = total;
        } else if total > second_highest_calorie_count {
            third_highest_calorie_count = second_highest_calorie_count;
            second_highest_calorie_count = total;
        } else if total > third_highest_calorie_count {
            third_highest_calorie_count = total;
        }
    }
    highest_calorie_count + second_highest_calorie_count + third_highest_calorie_count
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let answer_to_part_one = part_one(input);
    let answer_to_part_two = part_two(input);
    println!("ANSWER TO PART ONE: {answer_to_part_one}");
    println!("ANSWER TO PART TWO: {answer_to_part_two}");
}
