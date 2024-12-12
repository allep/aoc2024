use csv::Reader;
use serde::de::DeserializeOwned;
use std::fs;
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

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    let raw_content = fs::read_to_string(config.puzzle_input)?;
    let result: u32 = 0;
    Ok((result))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn count_trailhead_test() {
        todo!();
    }
}
