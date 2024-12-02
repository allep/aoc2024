use csv::Reader;
use serde::de::DeserializeOwned;
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

fn get_values_from_line(line: &str) -> Vec<i32> {
    let chunks = line.trim().split_whitespace();
    chunks
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn is_safe(values: &Vec<i32>) -> bool {
    are_not_oscillating(values) && are_all_gradual_changing(values)
}

fn is_safe_loose(values: &Vec<i32>) -> bool {
    if is_safe(values) {
        return true;
    }

    let mut loose_attempts = Vec::new();
    let mut is_loosely_valid = false;
    let _: Vec<_> = values
        .iter()
        .map(|x| {
            let mut v = values.clone();
            v.remove(v.iter().position(|y| *y == *x).expect("Element not found"));

            // now validate the remaining vector
            let safe = is_safe(&v);

            if !safe {
                loose_attempts.push(v.clone());
            }

            is_loosely_valid |= safe;
        })
        .collect();

    if !is_loosely_valid {
        println!("Found invalid: original = {values:?}\n    attempts = {loose_attempts:?}");
    }

    is_loosely_valid
}

fn are_not_oscillating(values: &[i32]) -> bool {
    values.iter().is_sorted() || values.iter().rev().is_sorted()
}

fn are_all_gradual_changing(values: &[i32]) -> bool {
    let mut all_gradual_changing = true;
    let _ = values
        .windows(2)
        .map(|w| {
            let diff = (w[0] - w[1]).abs();
            if diff < 1 || diff > 3 {
                all_gradual_changing = false;
            }
        })
        .collect::<Vec<_>>();
    all_gradual_changing
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

pub fn run(config: Config) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(config.puzzle_input)?;

    let lines = get_lines(&content);
    let mut num_safe = 0;
    let mut num_loosely_safe = 0;
    for line in lines {
        let values = get_values_from_line(line);
        if is_safe(&values) {
            num_safe += 1;
        }

        if is_safe_loose(&values) {
            num_loosely_safe += 1;
        }
    }

    Ok((num_safe, num_loosely_safe))
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
    fn day2_validate_not_oscillating() {
        let data = vec![7, 6, 4, 2, 1];
        assert!(are_not_oscillating(&data));
    }

    #[test]
    fn day2_validate_oscillating() {
        let data = vec![7, 6, 8, 3, 1];
        assert!(!are_not_oscillating(&data));
    }

    #[test]
    fn day2_validate_gradual() {
        let data = vec![1, 2, 3, 4, 5];
        assert!(are_all_gradual_changing(&data));
    }

    #[test]
    fn day2_validate_is_safe() {
        let data = vec![7, 6, 4, 2, 1];
        assert!(is_safe(&data));
    }

    #[test]
    fn day2_validate_is_safe_sample_input() {
        let data = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let lines = get_lines(data);
        let mut num_safe = 0;
        for line in lines {
            let values = get_values_from_line(line);
            if is_safe(&values) {
                println!("Found safe: {values:?}");
                num_safe += 1;
            } else {
                println!("Found unsafe: {values:?}");
            }
        }

        assert_eq!(num_safe, 2);
    }

    #[test]
    fn day2_validate_is_safe_loose_sample_input() {
        let data = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let lines = get_lines(data);
        let mut num_safe = 0;
        for line in lines {
            let values = get_values_from_line(line);
            if is_safe_loose(&values) {
                println!("Found safe: {values:?}");
                num_safe += 1;
            } else {
                println!("Found unsafe: {values:?}");
            }
        }

        assert_eq!(num_safe, 4);
    }

    #[test]
    fn day2_validate_special_loose_cases() {
        let data = "\
75 78 81 82 80";
        let values = get_values_from_line(data.trim());
        assert!(is_safe_loose(&values));
    }
}
