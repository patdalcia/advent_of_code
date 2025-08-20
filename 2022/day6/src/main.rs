use std::collections::HashSet;

fn has_duplicates<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let mut seen = HashSet::new();
    for item in vec {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}

fn part_one(input: &str) -> u32 {
    let mut answer = 0;
    let mut char_searcher: Vec<char> = vec![];
    for ch in input.chars() {
        if ch == '\n' || ch.is_whitespace() {
            answer += 1;
            continue;
        }
        if char_searcher.len() < 4 {
            char_searcher.push(ch);
            answer += 1;
            continue;
        }
        if !has_duplicates(&char_searcher) {
            return answer;
        }
        answer += 1;
        char_searcher.remove(0);
        char_searcher.push(ch);
    }
    0
}

fn part_two(input: &str) -> u32 {
    let mut answer = 0;
    let mut char_searcher: Vec<char> = vec![];
    for ch in input.chars() {
        if ch == '\n' || ch.is_whitespace() {
            answer += 1;
            continue;
        }
        if char_searcher.len() < 14 {
            char_searcher.push(ch);
            answer += 1;
            continue;
        }
        if !has_duplicates(&char_searcher) {
            return answer;
        }
        answer += 1;
        char_searcher.remove(0);
        char_searcher.push(ch);
    }
    0
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
