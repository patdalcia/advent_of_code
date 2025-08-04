fn generate_binary_combinations(n: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let total = 1 << n; // 2^n combinations

    for i in 0..total {
        let mut combo = Vec::with_capacity(n);
        for j in (0..n).rev() {
            let bit = (i >> j) & 1;
            combo.push(bit as u8);
        }
        result.push(combo);
    }

    result
}

fn main() {
    let combinations = generate_binary_combinations(4);
    println!("{combinations:?}");

    // TODO: create math struct, loop through combinations, figure out how to do the math from left
    // to right not by normal rules. GOODLUCK
}
