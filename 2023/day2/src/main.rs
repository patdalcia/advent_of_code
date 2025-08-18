struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn part_one(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut game_number = 0;
    let mut number_to_be_indexed = 0;
    let mut color_as_index = "";
    let mut match_counter = 0;
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut matched_lines = vec![];

    let iter = input.trim().split("\n");
    'line_loop: for line in iter {
        if let Some((before_colon, after_colon)) = line.split_once(':') {
            for word in before_colon.split_whitespace() {
                if let Ok(number) = word.parse::<i32>() {
                    game_number = number;
                    break;
                }
            }
            for individual_draws in after_colon.split(';') {
                for draw in individual_draws.split_whitespace() {
                    if let Ok(number) = draw.parse::<i32>() {
                        number_to_be_indexed = number;
                        continue;
                    } else {
                        color_as_index = draw.trim_matches(',');

                        match color_as_index {
                            "red" => {
                                if (bag.red - number_to_be_indexed) < 0 {
                                    continue 'line_loop;
                                }
                            }
                            "green" => {
                                if (bag.green - number_to_be_indexed) < 0 {
                                    continue 'line_loop;
                                }
                            }
                            "blue" => {
                                if (bag.blue - number_to_be_indexed) < 0 {
                                    continue 'line_loop;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            match_counter += game_number;
            matched_lines.push(line);
        }
    }
    Ok(match_counter)
}

fn part_two(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut game_number = 0;
    let mut number_to_be_indexed = 0;
    let mut color_as_index = "";
    let mut match_counter = 0;
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut matched_lines = vec![];
    let mut red_high = 0;
    let mut green_high = 0;
    let mut blue_high = 0;

    let iter = input.trim().split("\n");
    'line_loop: for line in iter {
        if let Some((before_colon, after_colon)) = line.split_once(':') {
            for word in before_colon.split_whitespace() {
                if let Ok(number) = word.parse::<i32>() {
                    game_number = number;
                    break;
                }
            }
            for individual_draws in after_colon.split(';') {
                for draw in individual_draws.split_whitespace() {
                    if let Ok(number) = draw.parse::<i32>() {
                        number_to_be_indexed = number;
                        continue;
                    } else {
                        color_as_index = draw.trim_matches(',');

                        match color_as_index {
                            "red" => {
                                if number_to_be_indexed > red_high {
                                    red_high = number_to_be_indexed;
                                }
                            }
                            "green" => {
                                if number_to_be_indexed > green_high {
                                    green_high = number_to_be_indexed;
                                }
                            }
                            "blue" => {
                                if number_to_be_indexed > blue_high {
                                    blue_high = number_to_be_indexed;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            let power = red_high * green_high * blue_high;
            match_counter += power;
            matched_lines.push(line);
            red_high = 0;
            green_high = 0;
            blue_high = 0;
        }
    }
    Ok(match_counter)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../puzzle_input.txt");
    let input_test = include_str!("../puzzle_input_test.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input)?);
    println!("ANSWER TO PART TWO: {}", part_two(input)?);

    Ok(())
}
