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

fn get_lines(raw_input: &str) -> Vec<&str> {
    let chunks: Vec<&str> = raw_input.trim().split("\n").collect();
    chunks
}

fn get_values_from_line(line: &str) -> Vec<i32> {
    let chunks = line.trim().split_whitespace();
    chunks
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn is_safe(values: Vec<i32>) -> bool {
    todo!();
}

fn are_not_oscillating(values: &[i32]) -> bool {
    todo!();
}

fn are_all_gradual(values: &[i32]) -> bool {
    todo!();
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
    fn day2_validate_parse_sample_input() {
        let data = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let lines = get_lines(data);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn day2_validate_parse_line() {
        let data = "7 6 4 2 1";

        let values = get_values_from_line(data);
        assert_eq!(values, vec![7, 6, 4, 2, 1]);
    }

    #[test]
    fn day2_validate_is_safe() {
        let data = vec![7, 6, 4, 2, 1];
        assert!(is_safe(data));
    }
}
