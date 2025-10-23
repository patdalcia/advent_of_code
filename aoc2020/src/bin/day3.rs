use std::{collections::HashMap, vec};

use aoc2020::get_puzzle;

fn print_grid(grid: &HashMap<(usize, usize), char>, row_limit: &usize, col_limit: &usize) {
    for r in 0..*row_limit {
        for c in 0..*col_limit {
            if let Some(ch) = grid.get(&(c, r)) {
                print!("{ch}");
            }
        }
        println!(" ");
    }
}

fn solve_puzzle(input: &str, xrow: usize, xcol: usize) -> u64 {
    println!("{input}");
    let mut grid = HashMap::new();
    let mut row_limit: usize = 0;
    let mut col_limit = 0;
    for (row, row_contents) in input.trim().lines().enumerate() {
        for (col, col_contents) in row_contents.chars().enumerate() {
            grid.insert((col, row), col_contents);
        }
        col_limit = row_contents.len();
        row_limit = row;
    }
    let mut curr_col: usize = 0;
    let mut curr_row: usize = 0;
    let mut tree_count = 0;

    while curr_row <= row_limit {
        if let Some(ch) = grid.get_mut(&(curr_col, curr_row)) {
            if *ch == '#' {
                tree_count += 1;
                *ch = 'X';
            } else {
                *ch = 'O';
            }
            // Making a step
            curr_row += xrow;
            curr_col += xcol;
            // Wrap Around Logic
            if curr_col >= col_limit {
                let temp_x = curr_col - col_limit;
                curr_col = temp_x;
            }
        }
    }
    // Print Logic
    print_grid(&grid, &row_limit, &col_limit);
    tree_count
}

fn main() {
    match get_puzzle("inputs/3.txt") {
        Ok(input) => {
            let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
            let mut answers = Vec::new();
            for slope in slopes {
                answers.push(solve_puzzle(&input, slope.1, slope.0));
            }
            let a1: u64 = solve_puzzle(&input, 1, 3);
            let a2: u64 = answers.iter().product();
            println!("ANSWER TO PART ONE: {a1}");
            println!("ANSWER TO PART TWO: {a2}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
