use csv::Reader;
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct Entry {
    output_start: i32,
    input_start: i32,
    input_range: i32,
}

#[derive(Debug)]
pub struct Config {
    first_file: String,
    second_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let first_file = args[1].clone();
        let second_file = args[2].clone();

        Ok(Config {
            first_file,
            second_file,
        })
    }
}

#[derive(Debug)]
struct WordSearch {
    lines: Vec<String>,
}

fn get_lines(raw_input: &str) -> Vec<&str> {
    let chunks: Vec<&str> = raw_input.trim().split("\n").collect();
    chunks
}

impl WordSearch {
    fn build(raw_content: &str) -> Result<WordSearch, &'static str> {
        let lines: Vec<String> = raw_content
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        Ok(WordSearch { lines })
    }

    fn compute(&self, word: &str) -> u32 {
        let key_letters = word.as_bytes();
        let positions = self.get_positions(key_letters[0] as char);
        for pos in positions {
            println!("Position found: ({}, {})", pos.0, pos.1);
        }

        0
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
}

fn deserialize<T, R>(reader: R) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::fmt::Debug + DeserializeOwned,
    R: Read,
{
    let mut rdr = Reader::from_reader(reader);
    let mut structs: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        structs.push(record);
    }

    Ok(structs)
}

fn compute_total_xmas(raw_data: &str) -> u32 {
    let word_search = WordSearch::build(raw_data).unwrap();
    word_search.compute("XMAS")
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

    #[test]
    fn part1_logic_test() {
        let data = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(compute_total_xmas(data), 18);
    }
}
