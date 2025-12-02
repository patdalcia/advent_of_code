use aoc2024::get_puzzle;

fn solve_puzzle(input: &str) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();
    let _ = input.lines().filter_map(|line| {
        let (a, b) = line.trim().split_once(' ')?;
        let a_parsed = a.parse::<i8>().ok()?;
        let b_parsed = b.parse::<i8>().ok()?;
        first_col.push(a_parsed);
        second_col.push(b_parsed);
        Some(())
    });
    first_col.sort();
    second_col.sort();

    let mut distance = 0;
    for i in 0..first_col.len() {
        println!("{} {}", first_col[i], second_col[i]);
    }
}

fn main() {
    match get_puzzle("inputs/1.txt") {
        Ok(input) => {
            solve_puzzle(input.as_str());
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    };
}
