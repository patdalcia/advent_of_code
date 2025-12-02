use std::collections::HashMap;

use aoc2024::get_puzzle;

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;
const MAP_MIDPOINT_X: i32 = MAP_WIDTH / 2;
const MAP_MIDPOINT_Y: i32 = MAP_HEIGHT / 2;
const SECONDS: u32 = ;

fn make_robots(input: String) -> Vec<Robot> {
    println!("INPUT -> {input}\n~MAKING ROBOTS BE PATIENT~");
    let mut robots = Vec::new();
    for line in input.lines() {
        if let Some(split_line) = line.split_once(' ') {
            if let Some(position_not_parsed_not_split) = split_line.0.split_once(',')
                && let Some(position_not_parsed) = position_not_parsed_not_split.0.split_once('=')
                && let Some(velocity_not_split) = split_line.1.split_once(',')
                && let Some(velocity_split) = velocity_not_split.0.split_once('=')
            {
                let unparsed_x = position_not_parsed.1;
                let unparsed_y = position_not_parsed_not_split.1;
                let unparsed_velocity_x = velocity_split.1;
                let unparsed_velocity_y = velocity_not_split.1;
                if let Ok(temp_position_x) = unparsed_x.parse::<i32>()
                    && let Ok(temp_position_y) = unparsed_y.parse::<i32>()
                    && let Ok(temp_velocity_x) = unparsed_velocity_x.parse::<i32>()
                    && let Ok(temp_velocity_y) = unparsed_velocity_y.parse::<i32>()
                {
                    let robot: Robot = Robot {
                        position: Point {
                            x: temp_position_x,
                            y: temp_position_y,
                        },
                        velocity: Point {
                            x: temp_velocity_x,
                            y: temp_velocity_y,
                        },
                    };
                    robots.push(robot);
                }
            }
        }
    }
    println!("~ROBOTS HAVE BEEN MADE~");
    robots
}

fn print_grid(robots: &Vec<Robot>, current_second: u32) {
    println!("~CURRENT SECOND -> {current_second}~");
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let current_point = Point { x: x, y: y };
            let mut to_print = '.';
            for robot in robots {
                if current_point == robot.position {
                    to_print = 'X';
                    break;
                }
            }
            print!("{to_print}");
        }
        println!(" ");
    }
}

fn solve_puzzle(mut robots: Vec<Robot>, seconds: u32) -> i32 {
    println!("~SOLVING PUZZLE BE PATIENT~");
    for second in 0..seconds {
        for robot in robots.iter_mut() {
            let dx = robot.velocity.x;
            let dy = robot.velocity.y;

            let mut position_x = robot.position.x;
            let mut position_y = robot.position.y;

            position_x += dx;
            position_y += dy;

            position_x = (position_x % MAP_WIDTH + MAP_WIDTH) % MAP_WIDTH;
            position_y = (position_y % MAP_HEIGHT + MAP_HEIGHT) % MAP_HEIGHT;

            robot.position.x = position_x;
            robot.position.y = position_y;
        }
        print_grid(&robots, second);
    }
    let mut final_coords = HashMap::new();
    for robot in robots {
        let final_x = robot.position.x;
        let final_y = robot.position.y;
        let final_position = Point {
            x: final_x,
            y: final_y,
        };
        final_coords
            .entry(final_position)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    final_coords.retain(|k, _| k.x != MAP_MIDPOINT_X && k.y != MAP_MIDPOINT_Y);

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for (key, value) in final_coords {
        if key.x < MAP_MIDPOINT_X && key.y < MAP_MIDPOINT_Y {
            tl += value;
        }
        if key.x > MAP_MIDPOINT_X && key.y < MAP_MIDPOINT_Y {
            tr += value;
        }
        if key.x < MAP_MIDPOINT_X && key.y > MAP_MIDPOINT_Y {
            bl += value;
        }
        if key.x > MAP_MIDPOINT_X && key.y > MAP_MIDPOINT_Y {
            br += value;
        }
    }
    println!("~ANSWER FOUND~");
    println!("TL:{tl} TR: {tr} BL:{bl} BR:{br}");

    tl * tr * bl * br
}

fn main() {
    match get_puzzle("inputs/14.txt") {
        Ok(input) => {
            let mut robots = make_robots(input);
            let answer = solve_puzzle(robots, SECONDS);
            println!("ANSWER TO PART ONE: {answer}");
        }
        Err(e) => {
            println!("ERROR: {e}")
        }
    }
}
