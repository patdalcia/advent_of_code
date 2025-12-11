use std::fs;
use std::io::Result;

pub fn get_puzzle(path: &str) -> Result<String> {
    fs::read_to_string(path)
}

fn concat_u32(a: u32, b: u32) -> u32 {
    a * 10u32.pow(b.ilog10() + 1) + b
}
