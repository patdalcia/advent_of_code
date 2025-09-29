use aoc2021::get_puzzle;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct FoldInfo {
    axis: String,
    index: usize,
}

fn remove_dups(grid: &mut Vec<Point>) -> Vec<Point> {
    let mut temp = Vec::new();
    for g in grid {
        if !temp.contains(g) {
            temp.push(g.clone());
        }
    }
    temp
}

fn print_grid(grid: &[Point]) {
    let mut max_x = 0;
    let mut max_y = 0;
    for p in grid {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    let width = max_y + 1;
    let height = max_x + 1;

    // Initialize with '.'
    let mut printable_grid: Vec<Vec<char>> = vec![vec!['_'; width]; height];

    // Mark points
    for p in grid {
        // Should be safe if your points are within bounds
        printable_grid[p.x][p.y] = '#';
    }

    println!("\n");
    for row in &printable_grid {
        for &c in row {
            print!("{c} ");
        }
        println!();
    }
}

fn solve_puzzle(input: &str) -> usize {
    let mut grid: Vec<Point> = Vec::new();
    let mut folds: Vec<FoldInfo> = Vec::new();
    for line in input.trim().lines() {
        if let Some(l) = line.trim().split_once(',') {
            if let (Ok(x_parsed), Ok(y_parsed)) = (l.0.parse::<usize>(), l.1.parse::<usize>()) {
                grid.push(Point {
                    x: x_parsed,
                    y: y_parsed,
                });
            }
        } else if let Some(fold) = line.split_whitespace().last() {
            if let Some(f) = fold.split_once('=') {
                if let Ok(num) = f.1.parse::<usize>() {
                    folds.push(FoldInfo {
                        axis: f.0.to_string(),
                        index: num,
                    });
                }
            }
        }
    }
    for fold in &folds {
        match fold.axis.as_str() {
            "y" => {
                for dot in grid.iter_mut() {
                    if dot.y > fold.index {
                        let distance_from_fold = dot.y.max(fold.index) - dot.y.min(fold.index);
                        dot.y -= distance_from_fold * 2;
                    }
                }
            }
            "x" => {
                for dot in grid.iter_mut() {
                    if dot.x > fold.index {
                        let distance_from_fold = dot.x.max(fold.index) - dot.x.min(fold.index);
                        dot.x -= distance_from_fold * 2;
                    }
                }
            }
            _ => {}
        }
    }
    let de_duped_grid = remove_dups(&mut grid);

    print_grid(&de_duped_grid);
    de_duped_grid.len()
}

fn main() {
    match get_puzzle("inputs/13.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input.as_str());
            println!("\n\nANSWER TO PART ONE: {answer}");
        }

        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
