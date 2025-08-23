use std::collections::HashSet;

struct Command {
    direction: char,
    move_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

struct Rope {
    head: Position,
    tail: Position,
}

struct KnottedRope {
    knots: Vec<Position>,
}

fn is_close(rope: &Rope) -> bool {
    let dx = (rope.head.row - rope.tail.row).abs();
    let dy = (rope.head.col - rope.tail.col).abs();
    dx <= 1 && dy <= 1
}

fn is_close_part_two(a: &Position, b: &Position) -> bool {
    let dx = (a.row - b.row).abs();
    let dy = (a.col - b.col).abs();
    dx <= 1 && dy <= 1
}

fn move_head_and_tail(rope: &mut Rope, command: &Command, been_here: &mut Vec<Position>) {
    let (dx, dy) = match command.direction {
        'R' => (0, 1),
        'U' => (-1, 0),
        'L' => (0, -1),
        'D' => (1, 0),
        _ => (0, 0),
    };

    for _ in 0..command.move_count {
        rope.head.row += dx;
        rope.head.col += dy;

        if !been_here.contains(&rope.tail) {
            been_here.push(rope.tail);
        }

        if !is_close(rope) {
            let dx = (rope.head.row - rope.tail.row).signum();
            let dy = (rope.head.col - rope.tail.col).signum();
            rope.tail.row += dx;
            rope.tail.col += dy;
        }
    }
}

fn move_rope(knotted_rope: &mut KnottedRope, command: &Command, been_here: &mut HashSet<Position>) {
    let (dx, dy) = match command.direction {
        'R' => (0, 1),
        'U' => (-1, 0),
        'L' => (0, -1),
        'D' => (1, 0),
        _ => (0, 0),
    };

    for _ in 0..command.move_count {
        let mut rope_iter = knotted_rope.knots.iter_mut();

        if let Some(head) = rope_iter.next() {
            head.row += dx;
            head.col += dy;

            let mut prev_knot = *head;

            for knot in rope_iter {
                if !is_close_part_two(knot, &prev_knot) {
                    let dx = (prev_knot.row - knot.row).signum();
                    let dy = (prev_knot.col - knot.col).signum();
                    knot.row += dx;
                    knot.col += dy;
                }
                prev_knot = *knot;
            }

            if let Some(tail) = knotted_rope.knots.last() {
                been_here.insert(*tail);
            }
        }
    }
}

fn part_one_test(input: &str) -> u32 {
    let commands: Vec<Command> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next()?.chars().next()?;
            let move_count = parts.next()?.parse::<u32>().ok()?;
            Some(Command {
                direction,
                move_count,
            })
        })
        .collect();

    let mut been_here: Vec<Position> = vec![];
    let mut rope = Rope {
        head: Position { row: 0, col: 0 },
        tail: Position { row: 0, col: 0 },
    };

    for command in &commands {
        move_head_and_tail(&mut rope, command, &mut been_here);
    }

    been_here.len() as u32
}

fn part_two(input: &str) -> u32 {
    let commands: Vec<Command> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next()?.chars().next()?;
            let move_count = parts.next()?.parse::<u32>().ok()?;
            Some(Command {
                direction,
                move_count,
            })
        })
        .collect();

    let mut been_here: HashSet<Position> = HashSet::new();
    let mut knotted_rope = KnottedRope {
        knots: vec![Position { row: 0, col: 0 }; 10],
    };

    been_here.insert(Position { row: 0, col: 0 });

    for command in &commands {
        move_rope(&mut knotted_rope, command, &mut been_here);
    }

    been_here.len() as u32
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one_test(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
