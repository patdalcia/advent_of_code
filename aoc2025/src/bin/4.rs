use aoc2025::get_puzzle;

const MAX_ADJ_PAPER_COUNT: u8 = 3;

#[derive(Debug, Default, Clone)]
struct Grid {
    array: Vec<char>,
    current_position: Point,
    line_length: usize,
}
impl Grid {
    fn check_point(&self) -> bool {
        let mut adj_paper_count = 0;
        let deltas = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, 1),
            (-1, -1),
            (1, 1),
            (1, -1),
        ];

        for d in deltas {
            let cur_x_signed = self.current_position.x as isize + d.0;
            let cur_y_signed = self.current_position.y as isize + d.1;

            if cur_x_signed < 0 || cur_y_signed < 0 {
                continue;
            }

            let cur_x = cur_x_signed as usize;
            let cur_y = cur_y_signed as usize;

            if cur_y >= self.array.len() / self.line_length {
                continue;
            }
            if cur_x >= self.line_length {
                continue;
            }

            let cur_index = cur_y * self.line_length + cur_x;
            if let Some(cur_ch) = self.array.get(cur_index)
                && *cur_ch == '@'
            {
                adj_paper_count += 1;
            }
            if adj_paper_count > MAX_ADJ_PAPER_COUNT {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Default, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn solve_puzzle(input: &str) -> i32 {
    println!("~~ INPUT ~~\n{input}");
    let mut part_one_answer = 0;

    let mut grid = Grid {
        line_length: input.trim().lines().next().unwrap().len(),
        array: input.trim().lines().flat_map(|line| line.chars()).collect(),
        current_position: Point::default(),
    };

    let line_len = grid.line_length;
    for (index, ch) in grid.array.iter().enumerate() {
        grid.current_position.x = index % line_len;
        grid.current_position.y = index / line_len;
        if *ch == '@' && grid.check_point() {
            part_one_answer += 1;
        }
    }
    part_one_answer
}

fn solve_puzzle_part_two(input: &str) -> i32 {
    let mut part_two_answer = 0;
    let mut answer_clone = -1;

    let mut grid = Grid {
        line_length: input.trim().lines().next().unwrap().len(),
        array: input.trim().lines().flat_map(|line| line.chars()).collect(),
        current_position: Point::default(),
    };

    let mut grid_clone = grid.clone();

    while part_two_answer != answer_clone {
        answer_clone = part_two_answer;

        let line_len = grid.line_length;
        for (index, ch) in grid.array.iter().enumerate() {
            grid.current_position.x = index % line_len;
            grid.current_position.y = index / line_len;
            if *ch == '@' && grid.check_point() {
                part_two_answer += 1;
                grid_clone.array[index] = '.';
            }
        }
        grid = grid_clone.clone();
    }
    part_two_answer
}

fn main() {
    match get_puzzle("inputs/4.txt") {
        Ok(input) => {
            let answer = solve_puzzle(&input);
            let answer2 = solve_puzzle_part_two(&input);
            println!("ANSWER TO PART ONE: {answer}\nANSWER TO PART TWO: {answer2}");
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}
