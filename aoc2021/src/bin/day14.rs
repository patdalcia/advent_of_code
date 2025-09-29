use std::collections::HashMap;

use aoc2021::get_puzzle;

fn get_count(polymer_template: &[char]) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for pt in polymer_template {
        counts.entry(*pt).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut most_count = 0;
    let mut least_count = 0;
    for count in counts {
        if least_count == 0 {
            least_count = count.1;
        }
        if count.1 > most_count {
            most_count = count.1;
        } else if count.1 < least_count {
            least_count = count.1;
        }
    }
    most_count - least_count
}

fn solve_puzzle(input: &str, steps: usize) -> usize {
    let mut insertion_rules: HashMap<(char, char), char> = HashMap::new();
    for line in input.lines().skip(2) {
        if let Some((lhs, rhs)) = line.split_once(" -> ") {
            let lhs = lhs.trim();
            let mut lhs_chars = lhs.chars();
            let a = lhs_chars.next().unwrap();
            let b = lhs_chars.next().unwrap();
            let ins = rhs.trim().chars().next().unwrap();
            insertion_rules.insert((a, b), ins);
        }
    }

    // Build initial pair counts from template
    let template = input.lines().next().unwrap();
    let template_chars: Vec<char> = template.chars().collect();

    let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();
    for pair in template_chars.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        *pair_counts.entry((a, b)).or_insert(0) += 1;
    }

    // Also keep track of counts of *individual* characters
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for c in &template_chars {
        *char_counts.entry(*c).or_insert(0) += 1;
    }

    for _step in 0..steps {
        let mut new_pair_counts: HashMap<(char, char), u64> = HashMap::new();

        for (&(a, b), &count) in pair_counts.iter() {
            if let Some(&ins) = insertion_rules.get(&(a, b)) {
                // rule (a,b) -> ins
                // new pairs are (a, ins) and (ins, b)
                *new_pair_counts.entry((a, ins)).or_insert(0) += count;
                *new_pair_counts.entry((ins, b)).or_insert(0) += count;

                // Update individual character count for the inserted char
                *char_counts.entry(ins).or_insert(0) += count;
            } else {
                // no insertion rule: keep the pair as is
                *new_pair_counts.entry((a, b)).or_insert(0) += count;
            }
        }

        pair_counts = new_pair_counts;
    }

    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    (max - min) as usize
}

fn main() {
    match get_puzzle("inputs/14.txt") {
        Ok(input) => {
            let answer1 = solve_puzzle(input.as_str(), 10);
            let answer2 = solve_puzzle(input.as_str(), 40);
            println!("ANSWER TO PART ONE: {answer1}");
            println!("ANSWER TO PART TWO: {answer2}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
