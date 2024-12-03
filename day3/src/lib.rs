use csv::Reader;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

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

fn compute_sum_of_products(addends: Vec<(i32, i32)>) -> i32 {
    let mut total = 0;
    addends
        .into_iter()
        .for_each(|pair| total += pair.0 * pair.1);
    total
}

fn parse_pair(pair: &str) -> (i32, i32) {
    let parts: Vec<&str> = pair.split(',').collect();

    assert_eq!(parts.len(), 2);

    (
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap(),
    )
}

fn parse_mul(mul: &str) -> Option<(i32, i32)> {
    let re = Regex::new(r"(?<values>\d+,\d+)").unwrap();

    if let Some(mat) = re.find(mul) {
        return Some(parse_pair(mat.as_str()));
    }

    None
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO
    // 1. get lines
    // 2.    for each line
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
    fn parse_sample_test() {
        let data = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let re = Regex::new(r"(?<block>mul\(\d+,\d+\))").unwrap();

        let mut muls = Vec::new();
        for mat in re.find_iter(data) {
            if let Some(values) = parse_mul(mat.as_str()) {
                muls.push(values);
            }
        }

        assert_eq!(muls, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);

        let total = compute_sum_of_products(muls);
        assert_eq!(total, 161);
    }
}