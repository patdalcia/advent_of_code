use std::collections::HashSet;
struct Direction {
    dx: i32,
    dy: i32,
}

struct PossibleMoves {
    directions: Vec<Direction>,
}

impl Default for PossibleMoves {
    fn default() -> Self {
        PossibleMoves {
            directions: vec![
                Direction { dx: -1, dy: 0 },  // up
                Direction { dx: 1, dy: 0 },   // down
                Direction { dx: 0, dy: -1 },  // left
                Direction { dx: 0, dy: 1 },   // right
                Direction { dx: -1, dy: -1 }, // up-left
                Direction { dx: -1, dy: 1 },  // up-right
                Direction { dx: 1, dy: -1 },  // down-left
                Direction { dx: 1, dy: 1 },   // down-right
            ],
        }
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for row in input.split('\n') {
        let mut char_list = vec![];
        for column in row.chars() {
            char_list.push(column);
        }
        grid.push(char_list);
    }
    grid
}

fn part_one(input: Vec<Vec<char>>) -> i32 {
    let mut answer = 0;
    let mut found_number_coords: Vec<(i32, i32)> = vec![];
    let possible_moves = PossibleMoves::default();
    for (row_index, row) in input.iter().enumerate() {
        for (col_index, column) in row.iter().enumerate() {
            if !(column.is_alphanumeric() || *column == '.') {
                //Found Symbol
                for direction in possible_moves.directions.iter() {
                    let dx = row_index as i32 + direction.dx;
                    let dy = col_index as i32 + direction.dy;
                    if dx < 0 //Checking bounds
                        || dy < 0
                        || dx as usize >= input.len()
                        || dy as usize >= input[0].len()
                    {
                        println!("ERROR: Out of bounds");
                        continue;
                    }
                    if input[dx as usize][dy as usize].is_numeric() {
                        found_number_coords.push((dx, dy));
                    }
                }
            }
        }
    }
    let mut globally_visited: HashSet<(usize, usize)> = HashSet::new();
    for coord in &found_number_coords {
        let x = coord.0 as usize;
        let y = coord.1 as usize;

        // Skip if already visited as part of another number
        if globally_visited.contains(&(x, y)) {
            continue;
        }

        let mut start_y = y;

        // Move left to find start of number
        while start_y > 0 && input[x][start_y - 1].is_ascii_digit() {
            start_y -= 1;
        }

        // Move right and collect digits + positions
        let mut num_str = String::new();
        let mut visited = HashSet::new();

        let mut curr_y = start_y;
        while curr_y < input[x].len() && input[x][curr_y].is_ascii_digit() {
            num_str.push(input[x][curr_y]);
            visited.insert((x, curr_y));
            curr_y += 1;
        }

        // Avoid processing digits again
        globally_visited.extend(&visited);

        answer += num_str.parse::<i32>().unwrap();
    }
    answer
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER: {}", part_one(get_grid(input)));
}
