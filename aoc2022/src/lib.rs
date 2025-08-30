use std::fs;
use std::io::Result;

pub fn get_puzzle(path: &str) -> Result<String> {
    fs::read_to_string(path)
}
