use aoc2021::get_puzzle;

#[derive(Debug)]
struct Jellyfish {
    power: u8,
    flashed: bool,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Point> {
        let x0 = self.x;
        let y0 = self.y;
        // Define the 8 delta offsets
        let deltas = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        deltas.into_iter().map(move |(dx, dy)| Point {
            x: x0 + dx,
            y: y0 + dy,
        })
    }

    fn in_bounds(&self, grid: &[Vec<Jellyfish>]) -> bool {
        if self.y < 0 || self.x < 0 {
            return false;
        }
        let yi = self.y as usize;
        let xi = self.x as usize;
        yi < grid.len() && xi < grid[yi].len()
    }
}

fn make_grid(input: &str) -> Vec<Vec<Jellyfish>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10))
                .map(|n| Jellyfish {
                    power: n as u8,
                    flashed: false,
                })
                .collect()
        })
        .collect()
}

fn flash(
    grid: &mut [Vec<Jellyfish>],
    flash_count: &mut u32,
    p: Point,
    sync_count: &mut u32,
    step_count: &u32,
) {
    const MAX_FLASH: usize = 99;
    let s_count: usize = grid
        .iter()
        .map(|line| line.iter().filter(|jelly| jelly.flashed).count())
        .sum();
    if s_count == MAX_FLASH {
        println!("SYNCRONIZED FLASH FOUND AT STEP -> {step_count}");
    }

    if !p.in_bounds(grid) {
        return;
    }
    let yi = p.y as usize;
    let xi = p.x as usize;
    let cell = &mut grid[yi][xi];
    if cell.flashed {
        return;
    }
    cell.flashed = true;
    *flash_count += 1;
    *sync_count += 1;

    for nb in p.neighbors() {
        if !nb.in_bounds(grid) {
            continue;
        }
        let ny = nb.y as usize;
        let nx = nb.x as usize;
        let adj = &mut grid[ny][nx];
        adj.power = adj.power.saturating_add(1);
        if adj.power > 9 && !adj.flashed {
            flash(grid, flash_count, nb, sync_count, step_count);
        }
    }
}

fn solve_puzzle(input: &str) -> u32 {
    let mut grid = make_grid(input);
    let mut flash_count = 0;
    let mut sync_count = 0;

    for step in 1..=300 {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                cell.power = cell.power.saturating_add(1);
            }
        }

        loop {
            let mut did_any = false;
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x].power > 9 && !grid[y][x].flashed {
                        let p = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                        flash(&mut grid, &mut flash_count, p, &mut sync_count, &step);
                        sync_count = 0;
                        did_any = true;
                    }
                }
            }

            if !did_any {
                break;
            }
        }

        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if cell.flashed {
                    cell.power = 0;
                }
                cell.flashed = false;
            }
        }
    }

    flash_count
}

fn main() {
    match get_puzzle("inputs/11.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input.as_str());
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
