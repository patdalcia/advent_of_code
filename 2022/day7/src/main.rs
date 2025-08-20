fn part_one(input: &str) -> u32 {
    for line in input.lines() {
        println!("{line}");
    }
    0
}

fn main() {
    let input = include_str!("../puzzle_input_test.txt");
    part_one(input);
}
