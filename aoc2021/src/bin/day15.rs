use aoc2021::get_puzzle;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

// We’ll store in the priority queue (cost so far, Position)
#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Position,
}

// For min‑heap behavior via max-heap, we wrap cost with Reverse
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We want min-heap by cost, so reverse the compare
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.y.cmp(&other.pos.y))
            .then_with(|| self.pos.x.cmp(&other.pos.x))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(pos: Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut res = Vec::new();
    if pos.x > 0 {
        res.push(Position {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < max_x {
        res.push(Position {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        res.push(Position {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < max_y {
        res.push(Position {
            x: pos.x,
            y: pos.y + 1,
        });
    }
    res
}

fn find_lowest_risk_dijkstra(grid: &[Vec<usize>]) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let dest = Position {
        x: width - 1,
        y: height - 1,
    };

    // distances: initialize with “infinite” cost
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; width]; height];
    // start at (0,0) with cost = 0
    dist[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        pos: Position { x: 0, y: 0 },
    });

    while let Some(State { cost, pos }) = heap.pop() {
        // If this cost is larger than the stored dist, we have already found a better path
        if cost > dist[pos.y][pos.x] {
            continue;
        }
        if pos == dest {
            // we reached the target with minimal cost
            return cost;
        }

        for nb in neighbors(pos, width - 1, height - 1) {
            let next_cost = cost + grid[nb.y][nb.x] as u32;
            if next_cost < dist[nb.y][nb.x] {
                dist[nb.y][nb.x] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    pos: nb,
                });
            }
        }
    }

    // If unreachable, return something
    dist[dest.y][dest.x]
}

fn solve_puzzle(input: &str) -> u32 {
    // Parse grid
    let risk_grid: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10))
                .map(|d| d as usize)
                .collect()
        })
        .collect();

    find_lowest_risk_dijkstra(&risk_grid)
}

fn main() {
    match get_puzzle("inputs/15.txt") {
        Ok(input) => {
            let answer1 = solve_puzzle(input.as_str());
            println!("ANSWER TO PART ONE: {answer1}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }
}
