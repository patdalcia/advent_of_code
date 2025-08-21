struct Move {
    dx: i32,
    dy: i32,
}

fn make_grid(input: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for row in input.lines() {
        let mut char_list = vec![];
        for col in row.chars() {
            if let Some(ch) = col.to_digit(10) {
                char_list.push(ch);
            }
        }
        grid.push(char_list);
    }
    grid
}

#[allow(unused_assignments)]
fn part_one(input: &str) -> u32 {
    let grid = make_grid(input);
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut answer = 0;
    let mut current_number = 0;

    let directions = vec![
        Move { dx: -1, dy: 0 }, // Up
        Move { dx: 1, dy: 0 },  // Down
        Move { dx: 0, dy: -1 }, // Left
        Move { dx: 0, dy: 1 },  // Right
    ];

    for row_index in 0..num_rows {
        'col_loop: for col_index in 0..num_cols {
            let start_row = row_index as i32;
            let start_col = col_index as i32;
            current_number = grid[row_index][col_index];
            for dir in &directions {
                let mut row = start_row;
                let mut col = start_col;
                loop {
                    row += dir.dx;
                    col += dir.dy;

                    // Stop if out of bounds
                    if row < 0 || row >= num_rows as i32 || col < 0 || col >= num_cols as i32 {
                        answer += 1;
                        continue 'col_loop;
                    }
                    let val = grid[row as usize][col as usize];
                    if val >= current_number {
                        break;
                    }
                }
            }
        }
    }
    answer
}

#[allow(unused_assignments)]
fn part_two(input: &str) -> u32 {
    let grid = make_grid(input);
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut answer = 0;
    let mut answer_list = vec![];
    let mut current_number = 0;

    let directions = vec![
        Move { dx: -1, dy: 0 }, // Up
        Move { dx: 1, dy: 0 },  // Down
        Move { dx: 0, dy: -1 }, // Left
        Move { dx: 0, dy: 1 },  // Right
    ];

    for row_index in 0..num_rows {
        for col_index in 0..num_cols {
            let start_row = row_index as i32;
            let start_col = col_index as i32;
            current_number = grid[row_index][col_index];
            for dir in &directions {
                let mut temp_answer = 0;
                let mut row = start_row;
                let mut col = start_col;
                loop {
                    row += dir.dx;
                    col += dir.dy;

                    // Stop if out of bounds
                    if row < 0 || row >= num_rows as i32 || col < 0 || col >= num_cols as i32 {
                        break;
                    }
                    let val = grid[row as usize][col as usize];
                    if val >= current_number {
                        temp_answer += 1;
                        break;
                    }
                    temp_answer += 1;
                }
                answer_list.push(temp_answer);
            }
            let product: u32 = answer_list.iter().product();
            if product > answer {
                answer = product;
            }
            answer_list.clear();
        }
    }
    answer
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
