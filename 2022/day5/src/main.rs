use std::vec;

fn part_one(input: &str) -> String {
    let mut crates_stripped = vec![];
    if let Some((crates, instructions)) = input.split_once("\n\n") {
        for line in crates.lines() {
            let mut to_be_pushed = vec![];
            for ch in line.chars() {
                if ch.is_alphabetic() {
                    to_be_pushed.push(ch);
                    println!("PUSHING: {ch}");
                }
            }
            crates_stripped.push(to_be_pushed);
        }
    }
    String::from("Test")
}
//TODO: Fix this mess, good luck ;(

fn main() {
    let input = include_str!("../puzzle_input_test.txt");
    println!("{}", part_one(input));
}
