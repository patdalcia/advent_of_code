fn char_score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn part_one(input: &str) -> u32 {
    let mut matches = vec![];
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        for ch in first.chars() {
            if let Some(matched_char) = second.find(ch) {
                // Found a match
                if let Some(m_char) = second.chars().nth(matched_char) {
                    if !(matches.contains(&m_char)) {
                        matches.push(m_char);
                    }
                }
            }
        }
        for m in &matches {
            total += char_score(*m);
        }
        matches.clear();
    }

    total
}

fn part_two(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut groups: Vec<Vec<&str>> = vec![];
    let mut total = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let group = vec![line1, line2, line3];
        groups.push(group);
    }
    for group in groups {
        let (line1, line2, line3) = (group[0], group[1], group[2]);
        for ch in line1.chars() {
            if line2.contains(ch) && line3.contains(ch) {
                // Match Found
                total += char_score(ch);
                break;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
