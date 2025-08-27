fn tick(input: &str) -> i32 {
    let mut tick_count = 0;
    let mut answer_vec = vec![];
    let mut tick_watcher = 20;
    let mut add_x_value = 1; // <- starts at 1 (important!)

    for line in input.lines() {
        if line == "noop" {
            tick_count += 1;
            if tick_count == 20 || tick_count == tick_watcher {
                answer_vec.push(add_x_value * tick_count);
                tick_watcher += 40;
            }
            continue;
        }

        let mut split_line = line.split_whitespace();
        if let Some(_command) = split_line.next() {
            if let Some(number_as_string) = split_line.next() {
                if let Ok(number) = number_as_string.parse::<i32>() {
                    for _ in 0..2 {
                        tick_count += 1;
                        if tick_count == 20 || tick_count == tick_watcher {
                            answer_vec.push(add_x_value * tick_count);
                            tick_watcher += 40;
                        }
                    }
                    // apply after both ticks
                    add_x_value += number;
                }
            }
        }
    }

    answer_vec.iter().sum()
}

fn cycle(print_position: &mut i32, pixel_position: i32, print_watcher: &mut i32) {
    let pp = *print_position;

    if pp > *print_watcher {
        *print_position = 0;
        if (pp - pixel_position).abs() <= 1 {
            print!("\n#");
        } else {
            print!("\n.");
        }
    } else {
        if (pp - pixel_position).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn tick_part2(input: &str) -> i32 {
    let mut cycle_count = 0;
    let mut pixel_position = 1;
    let mut print_position = 0;
    let mut print_watcher = 39;

    input.lines().for_each(|line| {
        if line != "noop" {
            let move_num: i32 = line.split_once(' ').unwrap().1.parse().unwrap();

            for _ in 0..2 {
                cycle_count += 1;
                cycle(&mut print_position, pixel_position, &mut print_watcher);
                print_position += 1;
            }
            pixel_position += move_num;
        } else {
            cycle_count += 1;
            cycle(&mut print_position, pixel_position, &mut print_watcher);
            print_position += 1;
        }
    });

    cycle_count // or whatever meaningful value you want to return
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER: {}", tick(input));
    tick_part2(input);
}
