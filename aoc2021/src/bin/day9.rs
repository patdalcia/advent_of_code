use aoc2021::get_puzzle;

fn solve_puzzle(input: &str) {
    println!("{input}");
    let complete_lines: Vec<Vec<char>> = input
        .trim()
        .lines()
        .filter(|line| {
            line.ends_with('>') || line.ends_with(')') || line.ends_with('}') || line.ends_with(']')
        })
        .map(|complete_line_as_str| complete_line_as_str.trim().chars().collect())
        .collect();
    for line in &complete_lines {
        let mut tag_count = 0;
        let mut chunk: Vec<char> = vec![];
        for ch in line {
            if *ch == '<' || *ch == '(' || *ch == '{' || *ch == '[' {
                tag_count += 1;
                chunk.push(*ch);
            } else if *ch == '>' || *ch == ')' || *ch == '}' || *ch == ']' {
                tag_count -= 1;
                chunk.push(*ch);
            }
            if tag_count <= 0 {
                // Corrupted Line
                println!("CHUNK FOUND");
                for c in &chunk {
                    print!("{c}")
                }
                println!(" ");
                chunk.clear();
                continue;
            }
        }
    }
    println!("{}", complete_lines.len());
}

fn main() {
    match get_puzzle("inputs/9.txt") {
        Ok(input) => {
            solve_puzzle(input.as_str());
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
