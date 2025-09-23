use aoc2021::get_puzzle;

fn solve_puzzle1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = input.lines().take(2).collect();
    let combined = lines.join(",");

    let numbers_to_be_drawn: Vec<u32> = combined
        .split(',')
        .filter_map(|num_as_str| num_as_str.trim().parse::<u32>().ok())
        .collect();

    let cards_as_str: Vec<&str> = input.split("\n\n").skip(2).collect();
    let mut cards: Vec<Vec<Vec<Option<u32>>>> = vec![];
    for card_as_str in cards_as_str {
        cards.push(
            card_as_str
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num_as_str| num_as_str.parse::<u32>().ok())
                        .collect::<Vec<Option<u32>>>()
                })
                .collect::<Vec<Vec<Option<u32>>>>(),
        );
    }

    let mut winning_card: Vec<Vec<Option<u32>>> = vec![];
    let mut winning_number = 0;

    'outer: for drawn_number in &numbers_to_be_drawn {
        for card in &mut cards {
            for card_row in card {
                for card_col in card_row {
                    if let Some(col) = *card_col {
                        if &col == drawn_number {
                            *card_col = None;
                        }
                    }
                }
            }
        }

        for card in &cards {
            for card_row in card {
                let non_none_count = card_row.iter().filter(|item| item.is_some()).count();
                if non_none_count == 0 {
                    winning_card = card.clone();
                    winning_number = *drawn_number;
                    break 'outer;
                }
            }
            if card.is_empty() || card[0].is_empty() {
                continue;
            }
            let num_rows = card.len();
            let num_cols = card[0].len();
            for col in 0..num_cols {
                let mut some_count = 0;
                for row in 0..num_rows {
                    if card[row][col].is_some() {
                        some_count += 1;
                    }
                }
                if some_count == 0 {
                    winning_card = card.clone();
                    winning_number = *drawn_number;
                    break 'outer;
                }
            }
        }
    }

    let mut card_score = 0;
    for row in winning_card {
        card_score += row.iter().filter_map(|num| *num).sum::<u32>();
    }

    Ok(card_score * winning_number)
}

fn solve_puzzle2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = input.lines().take(2).collect();
    let combined = lines.join(",");

    let numbers_to_be_drawn: Vec<u32> = combined
        .split(',')
        .filter_map(|num_as_str| num_as_str.trim().parse::<u32>().ok())
        .collect();

    let cards_as_str: Vec<&str> = input.split("\n\n").skip(1).collect();
    let mut cards: Vec<Vec<Vec<Option<u32>>>> = vec![];
    for card_as_str in cards_as_str {
        cards.push(
            card_as_str
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num_as_str| num_as_str.parse::<u32>().ok())
                        .collect::<Vec<Option<u32>>>()
                })
                .collect::<Vec<Vec<Option<u32>>>>(),
        );
    }

    let mut winning_card: Vec<Vec<Option<u32>>> = vec![];
    let mut winning_number = 0;
    let mut removed_cards = vec![false; cards.len()];

    'outer: for drawn_number in &numbers_to_be_drawn {
        for card in &mut cards {
            for card_row in card {
                for card_col in card_row {
                    if let Some(col) = *card_col {
                        if &col == drawn_number {
                            *card_col = None;
                        }
                    }
                }
            }
        }

        for (index, card) in cards.iter().enumerate() {
            if removed_cards[index] {
                continue;
            }
            for card_row in card {
                let non_none_count = card_row.iter().filter(|item| item.is_some()).count();
                if non_none_count == 0 {
                    winning_card = card.clone();
                    winning_number = *drawn_number;
                    removed_cards[index] = true;
                    if removed_cards.iter().filter(|&&x| x).count() == cards.len() {
                        break 'outer;
                    }
                    break;
                }
            }
            if removed_cards[index] {
                continue;
            }
            let num_rows = card.len();
            let num_cols = card[0].len();
            for col in 0..num_cols {
                let mut some_count = 0;
                for row in 0..num_rows {
                    if card[row][col].is_some() {
                        some_count += 1;
                    }
                }
                if some_count == 0 {
                    winning_card = card.clone();
                    winning_number = *drawn_number;
                    removed_cards[index] = true;
                    if removed_cards.iter().filter(|&&x| x).count() == cards.len() {
                        break 'outer;
                    }
                    break;
                }
            }
        }
    }

    let mut card_score = 0;
    for row in winning_card {
        card_score += row.iter().filter_map(|num| *num).sum::<u32>();
    }
    println!("PART TWO CARD SCORE AND WINNING NUM -> {card_score} {winning_number}");

    Ok(card_score * winning_number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_puzzle("inputs/4.txt")?;
    let answer1 = solve_puzzle1(input.as_str())?;
    let answer2 = solve_puzzle2(input.as_str())?;
    println!("ANSWER TO PART ONE: {answer1}");
    println!("ANSWER TO PART TWO: {answer2}");
    Ok(())
}
