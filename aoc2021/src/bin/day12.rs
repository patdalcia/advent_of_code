use std::collections::HashMap;

use aoc2021::get_puzzle;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cave {
    name: String,
    is_small: bool,
}

#[derive(Clone, Debug)]
struct CaveInfo {
    visited: bool,
    neighbors: Vec<Cave>,
    is_small: bool,
}

fn is_all_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn insert_neighbor(caves: &mut HashMap<String, CaveInfo>, cave_name: &str, neighbor: &Cave) {
    match caves.entry(cave_name.to_string()) {
        std::collections::hash_map::Entry::Occupied(mut ent) => {
            let cave_info = ent.get_mut();
            if !cave_info.neighbors.contains(neighbor) {
                cave_info.neighbors.push(neighbor.clone());
            }
        }
        std::collections::hash_map::Entry::Vacant(ent) => {
            ent.insert(CaveInfo {
                visited: false,
                neighbors: vec![neighbor.clone()],
                is_small: is_all_lowercase(cave_name),
            });
        }
    }
}

fn get_caves(input: &str, caves: &mut HashMap<String, CaveInfo>) {
    for line in input.trim().lines() {
        if let Some((a, b)) = line.split_once('-') {
            let cave_a = Cave {
                name: a.to_string(),
                is_small: is_all_lowercase(a),
            };
            let cave_b = Cave {
                name: b.to_string(),
                is_small: is_all_lowercase(b),
            };

            insert_neighbor(caves, &cave_a.name, &cave_b);
            insert_neighbor(caves, &cave_b.name, &cave_a);
        }
    }
}

fn find_paths(caves: &mut HashMap<String, CaveInfo>, current: &str, path_count: &mut u32) {
    if current == "end" {
        *path_count += 1;
        return;
    }

    let neighbor_names: Vec<String> = {
        let info = match caves.get(current) {
            Some(info) => info,
            None => return,
        };
        info.neighbors.iter().map(|c| c.name.clone()).collect()
    };

    if let Some(cur_info) = caves.get_mut(current) {
        cur_info.visited = true;
    } else {
        return;
    }

    for next in neighbor_names {
        // Do not revisit start
        if next == "start" {
            continue;
        }

        if let Some(next_info) = caves.get(&next) {
            if next_info.visited && next_info.is_small {
                continue;
            }
        }

        find_paths(caves, &next, path_count);
    }

    // Backtrack: unmark visited for current
    if let Some(cur_info) = caves.get_mut(current) {
        cur_info.visited = false;
    }
}

fn solve_puzzle(input: &str) -> u32 {
    let mut caves: HashMap<String, CaveInfo> = HashMap::new();
    let mut path_count = 0;
    get_caves(input, &mut caves);
    let current_cave = "start";

    find_paths(&mut caves, current_cave, &mut path_count);
    path_count
}

fn main() {
    match get_puzzle("inputs/12.txt") {
        Ok(input) => {
            let answer = solve_puzzle(input.as_str());
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
