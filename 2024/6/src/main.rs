use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    row: i32,
    col: i32,
    dir: Direction,
}

fn get_map() -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let file = File::open("data/puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let map: Result<Vec<Vec<char>>, std::io::Error> = reader
        .lines()
        .map(|line_result| {
            let line = line_result?; // unpack Result<String>
            Ok(line.chars().collect::<Vec<char>>())
        })
        .collect();

    Ok(map?)
}

fn find_player(map: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == target {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

#[allow(unused_assignments)]
fn solve_maze(map: &mut Vec<Vec<char>>, mut row: i32, mut column: i32) -> bool {
    let mut visited_states = HashSet::new();
    let mut current_direction = Direction::Up;
    let (mut dx, mut dy);
    let mut turned;
    let (mut temp_row, mut temp_col);
    let mut counter = 0;
    let mut current_space_icon = '|';

    println!("SOLVING MAZE");

    // Check: is the starting position in bounds?
    if row < 0 || column < 0 || row as usize >= map.len() || column as usize >= map[0].len() {
        println!("Start position ({row}, {column}) is out of bounds. Aborting.");
        return false;
    }

    // Check: is the starting position on a wall?
    if map[row as usize][column as usize] == '#' {
        println!("Start position ({row}, {column}) is a wall. Aborting.");
        return false;
    }

    map[row as usize][column as usize] = current_space_icon;

    loop {
        let state = State {
            row,
            col: column,
            dir: current_direction,
        };

        if !visited_states.insert(state) {
            println!("Loop detected at ({row}, {column}) facing {current_direction:?}");
            return true;
        }

        match current_direction {
            Direction::Up => {
                dx = -1;
                dy = 0;
                turned = Direction::Right;
                current_space_icon = '|';
            }
            Direction::Down => {
                dx = 1;
                dy = 0;
                turned = Direction::Left;
                current_space_icon = '|';
            }
            Direction::Left => {
                dx = 0;
                dy = -1;
                turned = Direction::Up;
                current_space_icon = '<';
            }
            Direction::Right => {
                dx = 0;
                dy = 1;
                turned = Direction::Down;
                current_space_icon = '>';
            }
        }

        temp_row = row + dx;
        temp_col = column + dy;

        // Bounds check before accessing
        if temp_row < 0
            || temp_col < 0
            || temp_row as usize >= map.len()
            || temp_col as usize >= map[0].len()
        {
            println!("Out of bounds at ({temp_row}, {temp_col})");
            break;
        }

        // Safe indexing using get()
        match map
            .get(temp_row as usize)
            .and_then(|r| r.get(temp_col as usize))
        {
            Some(&'#') => {
                current_direction = turned;
                continue;
            }
            Some(_) => {
                row = temp_row;
                column = temp_col;
                map[row as usize][column as usize] = current_space_icon;
            }
            None => {
                println!("Out of bounds when accessing ({temp_row}, {temp_col})");
                break;
            }
        }
    }

    for line in map {
        for l in line {
            let temp = *l;
            if temp == '|' || temp == '<' || temp == '>' {
                counter += 1;
            }
        }
    }

    println!("I VISITED~ {counter} ~UNIQUE SPACES TO SOLVE THE MAZE");
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = get_map()?;
    let mut loop_count = 0;
    // let test = take_a_walk(&mut map?);
    if let Some((mut row, mut column)) = find_player(&map, '^') {
        // walk(&mut map, row as i32, column as i32);

        for r_idx in 0..map.len() {
            println!("Starting Outer Loop");
            for c_idx in 0..map[0].len() {
                println!("Starting Inner Loop");
                let original = map[r_idx][c_idx];
                if map.is_empty() || map[0].is_empty() {
                    println!("Map is empty or malformed. Aborting count.");
                    break;
                }

                // Skip walls and the starting position
                if original == '#' || (r_idx as i32 == row as i32 && c_idx as i32 == column as i32)
                {
                    continue;
                }

                // Temporarily place a wall
                map[r_idx][c_idx] = '#';

                // Clone the map so solve_maze can mutate it
                let mut test_map = map.clone();
                if solve_maze(&mut test_map, row as i32, column as i32) {
                    loop_count += 1;
                }

                // Restore the original value
                map[r_idx][c_idx] = original;
                println!("LOOP COUNT: {loop_count:#?}");
            }
        }

        println!("LOOP COUNT: {loop_count:#?}");
        // solve_maze(&mut map, row as i32, column as i32);
    }

    Ok(())
}
