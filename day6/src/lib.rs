use std::io::{self, Read};
use std::{error::Error, fs, fs::File, process};

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

fn get_lines(raw_input: &str) -> Vec<&str> {
    let chunks: Vec<&str> = raw_input.trim().split("\n").collect();
    chunks
}

fn compute_total_unique_positions(raw_data: &str) -> u32 {
    todo!();
}

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    let content = fs::read_to_string(config.puzzle_input)?;
    let total = compute_total_unique_positions(&content);

    Ok((total))
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(compute_total_unique_positions(data), 41);
    }
}
