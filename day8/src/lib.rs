use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let puzzle_input = args[1].clone();

        Ok(Config { puzzle_input })
    }
}

#[derive(Debug)]
struct AntennasMap {
    lines: Vec<String>,
}

impl AntennasMap {
    fn build(raw_content: &str) -> Result<AntennasMap, &'static str> {
        let lines: Vec<String> = raw_content
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        let antennas_positions = Self::compute_antenna_positions(&lines);

        Ok(AntennasMap { lines })
    }

    fn compute_antenna_positions(lines: &Vec<String>) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.char_indices() {
                if c.is_ascii_alphanumeric() {
                    positions.push((x, y));
                }
            }
        }

        positions
    }

    fn compute(&self, word: &str) -> u32 {
        let key_letters = word.as_bytes();
        let c = key_letters[0] as char;
        let positions = self.get_positions(c);

        todo!();
    }

    fn get_positions(&self, letter: char) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        self.lines
            .iter()
            .enumerate()
            .for_each(|(line_number, &ref line)| {
                line.match_indices(letter)
                    .for_each(|(column, &ref c)| positions.push((line_number, column)));
            });

        positions
    }

    fn get_letter(&self, position: (usize, usize)) -> char {
        let num_lines = self.lines.len();
        let line_length = self.lines[0].len();
        assert!(position.0 < num_lines, "x = {}", position.0);
        assert!(position.1 < line_length, "y = {}", position.1);

        self.lines[position.0].as_bytes()[position.1] as char
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;
}
