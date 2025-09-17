use std::fs;
use std::io::Result;

pub fn get_puzzle(path: &str) -> Result<String> {
    fs::read_to_string(path)
}

// Checks if number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}
