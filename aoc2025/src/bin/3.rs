use aoc2025::get_puzzle;

fn get_joltage_output(input: &str) -> u32 {
    println!("~~INPUT~~\n{input}");
    let mut joltage_highs: Vec<u32> = Vec::new();
    let power_banks: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|joltage_as_char| joltage_as_char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();
    for bank in power_banks {
        let mut bank_high = 0;
        let mut bank_second_high = 0;

        for (index, joltage) in bank.iter().enumerate() {
            if index < bank.len() - 1 {
                if *joltage > bank_high {
                    bank_high = *joltage;
                    bank_second_high = 0;
                } else if *joltage > bank_second_high {
                    bank_second_high = *joltage;
                }
            } else if *joltage > bank_second_high {
                bank_second_high = *joltage;
            }
        }
        joltage_highs.push(concat(bank_high, bank_second_high));
    }
    joltage_highs.iter().sum()
}

fn concat(a: u32, b: u32) -> u32 {
    a * 10u32.pow(b.ilog10() + 1) + b
}

fn main() {
    match get_puzzle("inputs/3.txt") {
        Ok(input) => {
            let joltage_output = get_joltage_output(&input);
            println!("ANSWER TO PART ONE: {joltage_output}");
        }
        Err(e) => {
            eprint!("ERROR: {e}");
        }
    }
}
