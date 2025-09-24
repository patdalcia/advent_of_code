use aoc2021::get_puzzle;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    fn get_left(&self) -> Point {
        Point {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }
    fn get_right(&self) -> Point {
        Point {
            x: self.x.saturating_add(1),
            y: self.y,
        }
    }
    fn get_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y.saturating_sub(1),
        }
    }
    fn get_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y.saturating_add(1),
        }
    }
    fn get_moves(&self) -> Vec<Point> {
        let mut possible_moves: Vec<Point> = vec![];
        let left = self.get_left();
        let right = self.get_right();
        let up = self.get_up();
        let down = self.get_down();
        possible_moves.push(left);
        possible_moves.push(right);
        possible_moves.push(up);
        possible_moves.push(down);
        possible_moves
    }
    fn in_bounds(&self, width: usize, height: usize) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;
        x < width && y < height
    }

    fn is_corner(&self, width: usize, height: usize) -> bool {
        if !self.in_bounds(width, height) {
            return false;
        }
        let x = self.x as usize;
        let y = self.y as usize;
        (x == 0 && y == 0)
            || (x == width.saturating_sub(1) && y == 0)
            || (x == 0 && y == height.saturating_sub(1))
            || (x == width.saturating_sub(1) && y == height.saturating_sub(1))
    }
    fn is_edge(&self, width: usize, height: usize) -> bool {
        if !self.in_bounds(width, height) {
            return false;
        }
        let x = self.x as usize;
        let y = self.y as usize;

        // First check boundary:
        if x == 0 || x == width.saturating_sub(1) || y == 0 || y == height.saturating_sub(1) {
            // If it is on a corner, exclude it:
            let is_corner = (x == 0 && y == 0)
                || (x == width.saturating_sub(1) && y == 0)
                || (x == 0 && y == height.saturating_sub(1))
                || (x == width.saturating_sub(1) && y == height.saturating_sub(1));
            return !is_corner;
        }

        false
    }
}

fn make_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|num_as_str| num_as_str.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_less_count(grid: &[Vec<u32>], current_position: &Point, width: usize, height: usize) -> u32 {
    let possible_moves: Vec<Point> = current_position.get_moves();

    let current_num = grid[current_position.y as usize][current_position.x as usize];
    let mut match_count = 0;
    for m in possible_moves {
        if !m.in_bounds(width, height) {
            continue;
        }
        let testing_num = grid[m.y as usize][m.x as usize];
        if testing_num > current_num {
            match_count += 1;
        }
    }
    match_count
}

fn find_basins(
    grid: &[Vec<u32>],
    pos: &Point,
    width: usize,
    height: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    // Convert u32 to usize carefully
    let ux = pos.x as usize;
    let uy = pos.y as usize;

    // Bounds check
    if ux >= width || uy >= height {
        return 0;
    }
    // Already visited?
    if visited[uy][ux] {
        return 0;
    }

    let current = grid[uy][ux];
    // Optionally, treat certain values as barriers, e.g. 9
    if current == 9 {
        return 0;
    }

    // Mark visited
    visited[uy][ux] = true;

    // Count this cell
    let mut size = 1;

    // Explore neighbors
    for m in pos.get_moves() {
        let mx = m.x as usize;
        let my = m.y as usize;
        if mx < width && my < height {
            let neighbor = grid[my][mx];
            if neighbor > current {
                size += find_basins(grid, &m, width, height, visited);
            }
        }
    }

    size
}
fn top_three(mut vec: Vec<usize>) -> Vec<usize> {
    // Sort ascending (default), then reverse to descending
    vec.sort(); // ascending
    vec.reverse(); // now descending
    vec.into_iter().take(3).collect()
}

fn solve_puzzle(input: &str) -> (u32, u32) {
    let grid = make_grid(input);
    let mut lows = vec![];
    let mut basins = vec![];

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            let current_index = Point {
                x: x as u32,
                y: y as u32,
            };

            if current_index.is_corner(width, height) {
                let less_count = get_less_count(&grid, &current_index, width, height);
                if less_count == 2 {
                    let low = grid[current_index.y as usize][current_index.x as usize];
                    lows.push(low);
                    let mut visited = vec![vec![false; width]; height];
                    let basin_size =
                        find_basins(&grid, &current_index, width, height, &mut visited);
                    basins.push(basin_size);
                }
            } else if current_index.is_edge(width, height) {
                let less_count = get_less_count(&grid, &current_index, width, height);
                if less_count == 3 {
                    let low = grid[current_index.y as usize][current_index.x as usize];
                    lows.push(low);
                    let mut visited = vec![vec![false; width]; height];
                    let basin_size =
                        find_basins(&grid, &current_index, width, height, &mut visited);
                    basins.push(basin_size);
                }
            } else {
                let less_count = get_less_count(&grid, &current_index, width, height);
                if less_count == 4 {
                    let low = grid[current_index.y as usize][current_index.x as usize];
                    lows.push(low);
                    let mut visited = vec![vec![false; width]; height];
                    let basin_size =
                        find_basins(&grid, &current_index, width, height, &mut visited);
                    basins.push(basin_size);
                }
            }
        }
    }
    let answer1 = lows.iter().map(|low| 1 + *low).sum();
    let top_three = top_three(basins);
    let answer2 = top_three.iter().map(|t| *t as u32).product();
    (answer1, answer2)
}

fn main() {
    match get_puzzle("inputs/8.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input.as_str());
            println!(
                "ANSWER TO PART ONE: {}\nANSWER TO PART TWO: {}",
                answer.0, answer.1
            );
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
