use aoc2022::get_puzzle;
use std::{collections::HashSet, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn points_on_segment(a: Point, b: Point) -> Vec<Point> {
    // assume horizontal or vertical
    let mut pts = Vec::new();
    if a.x == b.x {
        let x = a.x;
        let (y0, y1) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        for y in y0..=y1 {
            pts.push(Point { x, y });
        }
    } else if a.y == b.y {
        let y = a.y;
        let (x0, x1) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        for x in x0..=x1 {
            pts.push(Point { x, y });
        }
    } else {
        panic!("Non horizontal/vertical segment in input");
    }
    pts
}

fn build_rocks(input: &str) -> (HashSet<Point>, i32 /* max_y */) {
    let mut rocks = HashSet::new();
    let mut max_y = 0;
    // You might also want min_x / max_x but we can avoid needing that
    for line in input.lines() {
        let corners: Vec<Point> = line
            .split("->")
            .filter_map(|group| {
                let group = group.trim();
                let (before, after) = group.split_once(',')?;
                let b = before.trim().parse::<i32>().ok()?;
                let a = after.trim().parse::<i32>().ok()?;
                Some(Point { x: b, y: a })
            })
            .collect();
        for pair in corners.windows(2) {
            let a = pair[0];
            let b = pair[1];
            for p in points_on_segment(a, b) {
                if p.y > max_y {
                    max_y = p.y;
                }
                rocks.insert(p);
            }
        }
    }
    (rocks, max_y)
}

fn run_sand(rocks: &mut HashSet<Point>, max_y: i32) -> i32 {
    let sand_start = Point { x: 500, y: 0 };
    let mut sand_at_rest = 0;
    let floor_y = max_y + 2;

    'outer: loop {
        // If source is blocked already, stop
        if rocks.contains(&sand_start) {
            break 'outer;
        }

        let mut sand = sand_start;

        loop {
            // try down
            let down = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            if down.y < floor_y && !rocks.contains(&down) {
                sand = down;
                continue;
            }
            // try down-left
            let left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            if left.y < floor_y && !rocks.contains(&left) {
                sand = left;
                continue;
            }
            // try down-right
            let right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if right.y < floor_y && !rocks.contains(&right) {
                sand = right;
                continue;
            }
            // else it comes to rest
            rocks.insert(sand);
            sand_at_rest += 1;

            // If it comes to rest at the source, stop
            if sand == sand_start {
                break 'outer;
            }

            break;
        }
    }

    sand_at_rest
}

fn make_grid(rocks: &HashSet<Point>) {
    // First figure out the bounding box: min_x, max_x, max_y, min_y (maybe 0 for y start)
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut min_y = i32::MAX;

    for rock in rocks {
        if rock.x < min_x {
            min_x = rock.x;
        }
        if rock.x > max_x {
            max_x = rock.x;
        }
        if rock.y < min_y {
            min_y = rock.y;
        }
        if rock.y > max_y {
            max_y = rock.y;
        }
    }

    // You might want to include the sand / source positions too in the bounding box
    // if you are also printing them; for now this only uses rocks.
    // Also clamp min_y to 0 if you want to start from y=0 always:
    if min_y > 0 {
        min_y = 0;
    }

    // Add a margin if you want some padding around the rock extents
    let x_margin = 2; // number of columns padding on left/right
    let y_margin_top = 0;
    let y_margin_bottom = 0;

    let xmin = min_x - x_margin;
    let xmax = max_x + x_margin;
    let ymin = min_y - y_margin_top;
    let ymax = max_y + y_margin_bottom;

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if rocks.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_puzzle(input: String) {
    let (mut rocks, max_y) = build_rocks(input.as_str());
    make_grid(&rocks);
    let sand_at_rest = run_sand(&mut rocks, max_y);
    println!("ANSWER TO PART 2: {sand_at_rest}");
}

fn main() {
    match get_puzzle("inputs/14_test.txt") {
        Ok(input) => {
            solve_puzzle(input);
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}

