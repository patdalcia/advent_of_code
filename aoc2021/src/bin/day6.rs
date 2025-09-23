use aoc2021::get_puzzle;

const MAX_DAY_COUNT: u64 = 256;
const RESET_TIMER: usize = 6;

fn init_fishes(fish_start: &mut [u64]) -> [u64; 9] {
    let max_fs = 8;

    let mut fishes: [u64; 9] = [0; 9];

    // For each fs in fish_start, increment the count in fishes[fs]
    for &fs in fish_start.iter() {
        let idx = fs as usize;
        if let Some(fish) = fishes.get_mut(idx) {
            *fish += 1;
        }
    }
    print_fishes(&fishes);
    fishes
}

/// Panics if `counts.len() != 9`.
fn update_fishes(counts: &mut [u64; 9]) {
    let zero_count = counts[0];

    // shift timers down: timer 1→0, 2→1, …, 8→7
    counts.rotate_left(1);
    counts[RESET_TIMER] += zero_count;
    counts[8] = zero_count;
}

fn print_fishes(fishes: &[u64]) {
    for fish in fishes {
        print!("({fish})");
    }
    println!(" ");
}

fn solve_puzzle(input: String) -> usize {
    println!("{input}");
    let mut fish_start: Vec<u64> = input
        .trim()
        .split(',')
        .filter_map(|ch| ch.trim().parse::<u64>().ok())
        .collect();
    let mut fishes = init_fishes(&mut fish_start);
    print!("INITIAL STATE: ");
    for fish in &fish_start {
        print!("{fish}, ");
    }
    println!(" ");
    println!("*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*\n");
    for _ in 0..MAX_DAY_COUNT {
        update_fishes(&mut fishes);
    }
    fishes.iter().map(|fish| *fish as usize).sum()
}

fn main() {
    match get_puzzle("inputs/6.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input);
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
