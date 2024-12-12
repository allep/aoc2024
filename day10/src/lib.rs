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

struct TopographicMap {
    positions: Vec<Vec<char>>,
    x_max: usize,
    y_max: usize,
    trailheads: Vec<(usize, usize)>,
}

impl TopographicMap {
    pub fn make(raw_data: &str) -> Result<TopographicMap, &'static str> {
        let positions: Vec<Vec<char>> = raw_data
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();

        if positions.len() == 0 {
            return Err("No lines read from raw content.");
        }

        if positions[0].len() == 0 {
            return Err("Read empty line.");
        }

        let y_max = positions.len();
        let x_max = positions[0].len();

        Ok(TopographicMap {
            positions,
            x_max,
            y_max,
            trailheads: Vec::new(),
        })
    }

    fn compute_trailheads(&mut self) {
        for (y, line) in self.positions.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '0' {
                    self.trailheads.push((x, y));
                }
            }
        }
    }

    fn trailheads_num(&self) -> usize {
        self.trailheads.len()
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
        let data = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let mut topographic_map = TopographicMap::make(data).unwrap();
        topographic_map.compute_trailheads();
        let num_trailheads = topographic_map.trailheads_num();
        assert_eq!(num_trailheads, 9);
    }
}
