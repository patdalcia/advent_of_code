use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn get_lines() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn get_rules(mut lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    // Remove the last line if it's empty
    if let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        }
    }

    let mut rules: Vec<String> = vec![];
    let mut jobs: Vec<String> = vec![];
    let mut empty_index = 0;

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            empty_index = index;
            break;
        }
        rules.push(line.to_string());
    }
    jobs = lines[empty_index + 1..].to_vec();

    (rules, jobs)
}

fn check_matches(rules: &Vec<String>, jobs: &Vec<String>) -> (Vec<String>, Vec<String>, i32) {
    let mut matches = vec![];
    let mut misses = vec![];
    let mut center_count = 0;

    'job_loop: for job in jobs {
        // Parse the job line into a vector of numbers
        let job_as_numbers: Vec<i32> = job
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        'rule_loop: for rule in rules {
            let rule_as_numbers: Vec<i32> = rule
                .split('|')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            if rule_as_numbers.len() != 2 {
                continue 'rule_loop; // skip this malformed rule
            }
            if !contains_all(&job_as_numbers, &rule_as_numbers) {
                continue 'rule_loop; // skip irrelevant rule
            }
            let x = rule_as_numbers[0];
            let y = rule_as_numbers[1];

            for num in &job_as_numbers {
                if *num == y {
                    misses.push(job.to_string());
                    continue 'job_loop; // job is invalid for this rule
                }
                if *num == x {
                    break; // done checking this rule — go to next rule
                }
            }
        }
        center_count += get_middle(&job_as_numbers);
        matches.push(job.to_string());
    }
    (matches, misses, center_count)
}

//checks that vector a contains all elements of vector b
fn contains_all(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    let set_a: HashSet<_> = a.iter().collect();
    b.iter().all(|item| set_a.contains(item))
}

fn get_middle(job: &Vec<i32>) -> i32 {
    job[job.len() / 2]
}

fn fix(misses: &Vec<String>, rules: &Vec<String>) -> (Vec<String>, i32) {
    let mut fixed: Vec<String> = vec![];
    let mut middle_count = 0;

    for job in misses.iter() {
        let mut job_as_numbers: Vec<i32> = job
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        let mut rule_index = 0;
        while rule_index < rules.len() {
            let rule = &rules[rule_index];
            let rule_as_numbers: Vec<i32> = rule
                .split('|')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            if rule_as_numbers.len() != 2 {
                rule_index += 1;
                continue;
            }

            let x = rule_as_numbers[0];
            let y = rule_as_numbers[1];

            if !contains_all(&job_as_numbers, &rule_as_numbers) {
                rule_index += 1;
                continue;
            }

            let ix = job_as_numbers.iter().position(|&n| n == x);
            let iy = job_as_numbers.iter().position(|&n| n == y);

            if let (Some(i), Some(j)) = (ix, iy) {
                if i > j {
                    job_as_numbers.swap(i, j);
                    rule_index = 0; // Restart rule list after swap
                    continue;
                }
            }

            rule_index += 1;
        }

        middle_count += get_middle(&job_as_numbers);
        let line = job_as_numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        fixed.push(line);
    }

    (fixed, middle_count)
}

fn main() {
    println!("WORKING ON IT, DEPENDING ON INPUT SIZE IT MAY TAKE A FEW SECONDS");
    match get_lines() {
        Ok(lines) => {
            let (rules, jobs) = get_rules(lines.clone());
            let (matches, misses, center_count) = check_matches(&rules, &jobs);
            let (fixed, middle_count) = fix(&misses, &rules);
            println!("INITALLY CORRECT CENTER COUNT: {center_count:#?}");
            println!("FIXED MIDDLE COUNT: {middle_count:#?}");
        }
        Err(e) => {
            println!("ERROR: {e:#?}");
        }
    }
}
