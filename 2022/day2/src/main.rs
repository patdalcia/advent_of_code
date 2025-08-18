fn check_game_result(opponent_move_char: &char, player_move_char: &char) -> i32 {
    if player_move_char == opponent_move_char {
        //Tie
        return 0;
    }
    match player_move_char {
        'Y' => {
            //Paper
            if *opponent_move_char == 'C' {
                //loss
                return 2;
            }
            if *opponent_move_char == 'B' {
                // Tie
                return 0;
            }
            //win
            return 1;
        }
        'X' => {
            //Rock
            if *opponent_move_char == 'B' {
                //loss
                return 2;
            }
            if *opponent_move_char == 'A' {
                // Tie
                return 0;
            }
            //win
            return 1;
        }
        'Z' => {
            //Scissors
            if *opponent_move_char == 'A' {
                //loss
                return 2;
            }
            if *opponent_move_char == 'C' {
                // Tie
                return 0;
            }
            //win
            return 1;
        }
        _ => {}
    }
    0
}

fn part_one(input: &str) -> i32 {
    let mut total_score = 0;
    let mut current_game_score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        if let Some(opponent_move_char) = chars.next() {
            if chars.next().is_some() {
                if let Some(player_move_char) = chars.next() {
                    match player_move_char {
                        'Y' => {
                            //Paper
                            current_game_score += 2;
                            match check_game_result(&opponent_move_char, &player_move_char) {
                                0 => {
                                    //tie
                                    current_game_score += 3;
                                }
                                1 => {
                                    // win
                                    current_game_score += 6;
                                }
                                2 => {
                                    // loss
                                    current_game_score += 0;
                                }
                                _ => {}
                            }
                        }
                        'X' => {
                            //Rock
                            current_game_score += 1;
                            match check_game_result(&opponent_move_char, &player_move_char) {
                                0 => {
                                    //tie
                                    current_game_score += 3;
                                }
                                1 => {
                                    // win
                                    current_game_score += 6;
                                }
                                2 => {
                                    // loss
                                    current_game_score += 0;
                                }
                                _ => {}
                            }
                        }
                        'Z' => {
                            //Scissors
                            current_game_score += 3;
                            match check_game_result(&opponent_move_char, &player_move_char) {
                                0 => {
                                    //tie
                                    current_game_score += 3;
                                }
                                1 => {
                                    // win
                                    current_game_score += 6;
                                }
                                2 => {
                                    // loss
                                    current_game_score += 0;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        total_score += current_game_score;
        current_game_score = 0;
    }
    total_score
}

fn get_player_char(opponent_move_char: &char, intended_game_result_char: &char) -> char {
    match intended_game_result_char {
        'X' => {
            // Need a loss
            match opponent_move_char {
                'A' => 'Z',
                'B' => 'X',
                'C' => 'Y',
                _ => ' ',
            }
        }
        'Y' => {
            // Need a draw
            match opponent_move_char {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                _ => ' ',
            }
        }
        'Z' => {
            // Need a win
            match opponent_move_char {
                'A' => 'Y',
                'B' => 'Z',
                'C' => 'X',
                _ => ' ',
            }
        }
        _ => ' ',
    }
}

fn get_player_char_score(player_char: char) -> i32 {
    match player_char {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn part_two(input: &str) -> i32 {
    let mut total_score = 0;
    let mut current_game_score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        if let Some(opponent_move_char) = chars.next() {
            if chars.next().is_some() {
                if let Some(intended_game_result_char) = chars.next() {
                    match intended_game_result_char {
                        'X' => {
                            // Need to lose
                            current_game_score += 0;
                        }
                        'Y' => {
                            // Need a draw
                            current_game_score += 3;
                        }
                        'Z' => {
                            // Need a win
                            current_game_score += 6;
                        }
                        _ => {}
                    }
                    current_game_score += get_player_char_score(get_player_char(
                        &opponent_move_char,
                        &intended_game_result_char,
                    ));
                }
            }
        }
        total_score += current_game_score;
        current_game_score = 0;
    }
    total_score
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("ANSWER TO PART ONE: {}", part_one(input));
    println!("ANSWER TO PART TWO: {}", part_two(input));
}
