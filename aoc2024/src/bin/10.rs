use aoc2024::get_puzzle;

fn make_grid(input: &String) -> Vec<Vec<u32>> {
    println!("~~ Input ~~\n{input}");
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        grid.push(
            line.chars()
                .filter_map(|num_as_char| num_as_char.to_digit(16))
                .collect(),
        );
    }
    grid
}

fn walk_trail(grid: &[Vec<u32>], x: i32, y: i32, visited: &mut Vec<(i32, i32)>) -> i32 {
    let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let height = grid.len() as i32;
    if height == 0 {
        return 0;
    }
    let width = grid[0].len() as i32;

    let mut score = 0;
    let current = grid[y as usize][x as usize];

    for (dx, dy) in moves {
        let next_x = x + dx;
        let next_y = y + dy;
        if next_x >= 0 && next_x < width && next_y >= 0 && next_y < height {
            if visited.contains(&(next_x, next_y)) {
                continue;
            }
            let next = grid[next_y as usize][next_x as usize];
            if next == current + 1 {
                if next == 9 {
                    score += 1;
                    //visited.push((next_x, next_y));
                } else {
                    //visited.push((next_x, next_y));
                    score += walk_trail(grid, next_x, next_y, visited);
                }
            }
        }
    }
    score
}

fn solve_puzzle(grid: Vec<Vec<u32>>) -> i32 {
    let mut total_score = 0;
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let grid_clone = grid.clone();
    for (y, line) in grid_clone.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 0 {
                total_score += walk_trail(&grid, x as i32, y as i32, &mut visited);
                visited.clear();
            }
        }
    }
    total_score
}

fn main() {
    match get_puzzle("inputs/10.txt") {
        Ok(input) => {
            let trailhead_score = solve_puzzle(make_grid(&input));
            println!("TRAILHEAD SCORE: {trailhead_score}");
        }
        Err(e) => {
            eprint!("ERROR: {e}")
        }
    }
}
