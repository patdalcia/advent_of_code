use std::fs;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    if_true: usize,
    if_false: usize,
    inspect_count: u64,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Value),
    Mul(Value),
}

#[derive(Clone, Debug)]
enum Value {
    Old,
    Num(u64),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(val) => old + val.get(old),
            Operation::Mul(val) => old * val.get(old),
        }
    }
}

impl Value {
    fn get(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Num(n) => *n,
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|block| {
            let mut lines = block.lines().skip(1);

            let items_line = lines.next()?;
            let items: Vec<u64> = items_line
                .split(": ")
                .nth(1)?
                .split(", ")
                .filter_map(|s| s.parse().ok())
                .collect();

            let op_line = lines.next()?;
            let op_part = op_line.split("= old ").nth(1)?;
            let (op_char, operand_str) = op_part.split_once(' ')?;
            let operand = match operand_str {
                "old" => Value::Old,
                _ => Value::Num(operand_str.parse().ok()?),
            };
            let operation = match op_char {
                "+" => Operation::Add(operand),
                "*" => Operation::Mul(operand),
                _ => return None,
            };

            let test_line = lines.next()?;
            let test_divisible = test_line.rsplit_once(' ')?.1.parse().ok()?;

            let true_line = lines.next()?;
            let if_true = true_line.rsplit_once(' ')?.1.parse().ok()?;

            let false_line = lines.next()?;
            let if_false = false_line.rsplit_once(' ')?.1.parse().ok()?;

            Some(Monkey {
                items,
                operation,
                test_divisible,
                if_true,
                if_false,
                inspect_count: 0,
            })
        })
        .collect()
}

fn do_round_part_two(monkeys: &mut [Monkey], modulo_base: u64) {
    for i in 0..monkeys.len() {
        let items = std::mem::take(&mut monkeys[i].items);
        let monkey = monkeys[i].clone();

        for mut worry in items {
            worry = monkey.operation.apply(worry);
            // Reduce worry using modulo to avoid overflow
            worry %= modulo_base;
            let target = if worry % monkey.test_divisible == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };
            monkeys[i].inspect_count += 1;
            monkeys[target].items.push(worry);
        }
    }
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let mut monkeys = parse_monkeys(input);

    // Compute the product of all test divisors as the modulo base
    let modulo_base: u64 = monkeys.iter().map(|m| m.test_divisible).product();

    println!("Initial state:");
    for (idx, m) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", idx, m.items);
    }

    for round in 0..10000 {
        do_round_part_two(&mut monkeys, modulo_base);
    }

    let mut first_highest = 0;
    let mut second_highest = 0;
    for monkey in &monkeys {
        let count = monkey.inspect_count;
        if count > first_highest {
            second_highest = first_highest;
            first_highest = count;
        } else if count > second_highest {
            second_highest = count;
        }
    }

    println!("FIRST: {} SECOND: {}", first_highest, second_highest);
    println!("ANSWER: {}", first_highest * second_highest);
}
