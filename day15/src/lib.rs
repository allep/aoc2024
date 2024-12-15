use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input_map: String,
    puzzle_input_moves: String,
}

struct Moves {
    moves: Vec<char>,
}

impl Moves {
    pub fn make(raw_data: &str) -> Result<Moves, &'static str> {
        let moves: Vec<char> = raw_data
            .as_bytes()
            .iter()
            .map(|b| *b as char)
            .filter(|c| *c as char != '\n')
            .collect();

        let mut is_ok = true;
        moves.iter().for_each(|c| {
            if *c != '^' && *c != 'v' && *c != '<' && *c != '>' {
                is_ok = false;
            }
        });

        if !is_ok {
            return Err("Some moves were not acceptable");
        }

        Ok(Moves { moves })
    }
}

struct WarehouseMap {
    positions: Vec<Vec<char>>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let puzzle_input_map = args[1].clone();
        let puzzle_input_moves = args[2].clone();

        Ok(Config {
            puzzle_input_map,
            puzzle_input_moves,
        })
    }
}

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    Ok((0))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;
}
