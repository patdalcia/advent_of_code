use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{usize, vec};

fn get_file() -> Result<File, Box<dyn std::error::Error>> {
    let file_path = "data/puzzle_input.txt";
    let file = File::open(file_path)?;

    Ok(file)
}

fn find_x_mas(word: &str, word_search: &Vec<Vec<char>>) -> Vec<(usize, usize, usize, usize)> {
    let mut matches = vec![];
    let mut total = 0;
    let mut temp_word: Vec<(usize, usize)> = vec![];
    // edge cases
    if word.len() == 0 || word_search.len() == 0 || word_search[0].len() == 0 {
        return vec![];
    }

    let word_chars = word.chars().collect::<Vec<char>>();
    let word_len = word_chars.len() as isize;
    let word_search_width = word_search[0].len() as isize;
    let word_search_height = word_search.len() as isize;

    for y in 0..word_search_height {
        for x in 0..word_search_width {
            if word_search[y as usize][x as usize] != word_chars[0] {
                continue;
            }
            let temp = word_chars[0];
            println!("FOUND: {temp:#?}");
            total += 1;
            // loop through every direction that a word can go in
            let deltas = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
            'deltas_loop: for (dx, dy) in deltas {
                // check if direction would not end up oob
                let end_x = x + dx * (word_len - 1);
                let end_y = y + dy * (word_len - 1);
                if end_x >= word_search_width
                    || end_y >= word_search_height
                    || end_x < 0
                    || end_y < 0
                {
                    continue;
                }

                // loop through every character in the direction
                // and see if any of them don't match up with the word
                for i in 0..word_len {
                    if i == 0 {
                        temp_word.clear();
                    }
                    let xprime = x + dx * i;
                    let yprime = y + dy * i;
                    if word_search[yprime as usize][xprime as usize] != word_chars[i as usize] {
                        continue 'deltas_loop;
                    }
                    //char found saving location
                    temp_word.push((xprime as usize, yprime as usize));
                }
                let (x_end, y_end) = temp_word.last().unwrap();

                // word found! add to matches list
                matches.push((x as usize, y as usize, *x_end, *y_end));
            }
        }
    }
    matches
}

fn find_x(grid: &[String]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let target = "MAS";
    let target_rev = "SAM";
    let mut count = 0;

    for row in 0..(height - 2) {
        for col in 0..(width - 2) {
            let mut diagonal1 = String::new();
            let mut diagonal2 = String::new();

            for i in 0..3 {
                let c1 = grid[row + i].chars().nth(col + i).unwrap();
                let c2 = grid[row + i].chars().nth(col + 2 - i).unwrap();
                diagonal1.push(c1);
                diagonal2.push(c2);
            }

            if (diagonal1 == target || diagonal1 == target_rev)
                && (diagonal2 == target || diagonal2 == target_rev)
            {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let word = "MAS";
    match get_file() {
        Ok(file) => {
            let buf = BufReader::new(file);
            let lines: Vec<String> = buf.lines().collect::<Result<_, _>>()?;
            let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
            let total = find_x_mas(word, &grid);
            println!("{total:#?}");
            let t = find_x(&lines);
            println!("Total X-MAS: {t:#?}");
        }
        Err(e) => {
            println!("ERROR: {e:#?}");
        }
    }

    Ok(())
}
