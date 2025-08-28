use std::ops::AddAssign;

fn part_one(input: &str) -> u64 {
    let mut first_col: Vec<u64> = vec![];
    let mut second_col: Vec<u64> = vec![];
    input.lines().for_each(|line| {
        let mut line_iter = line.split_whitespace();
        if let Some(first_col_num) = line_iter.next() {
            if let Some(second_col_num) = line_iter.next() {
                if let Ok(first_num) = first_col_num.parse::<u64>() {
                    if let Ok(second_num) = second_col_num.parse::<u64>() {
                        first_col.push(first_num);
                        second_col.push(second_num);
                    }
                }
            }
        }
    });
    first_col.sort();
    second_col.sort();
    let mut answer = 0;
    first_col.iter().enumerate().for_each(|(index, first_num)| {
        println!("FIRST: {first_num} SECOND: {}", second_col[index]);
        let temp = if first_num > &second_col[index] {
            answer.add_assign(first_num - second_col[index]);
        } else {
            answer.add_assign(second_col[index] - first_num);
        };
    });
    answer
}
fn main() {
    let input = include_str!("../puzzle_input_test.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
}
