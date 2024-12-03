use csv::Reader;
use regex::Regex;
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

fn get_line_chunks(line: &str) -> Vec<&str> {
    let mut start = 0;
    let mut end = line.len() - 1;

    let mut chunks = Vec::new();
    let mut sub_line = line;

    let dont_tag = "don't()";
    let do_tag = "do()";

    loop {
        if let Some(end) = sub_line.find(dont_tag) {
            chunks.push(&sub_line[start..end]);

            sub_line = &sub_line[end + dont_tag.len()..];

            if let Some(start) = sub_line.find(do_tag) {
                sub_line = &sub_line[start + do_tag.len()..];
            } else {
                // after a don't I will have either a do or EOL
                break;
            }
        } else {
            // after a do I will have either a don't or EOL
            chunks.push(sub_line);
            break;
        }
    }

    chunks
}

pub fn run(config: Config) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(config.puzzle_input)?;
    let lines = get_lines(&content);

    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let mut total = 0;
    for line in &lines {
        let mut muls = Vec::new();
        for mat in re.find_iter(line) {
            if let Some(values) = parse_mul(mat.as_str()) {
                muls.push(values);
            }
        }

        total += compute_sum_of_products(muls);
    }

    let mut total_filtered = 0;
    for line in &lines {
        let chunks = get_line_chunks(line);

        let mut filtered_line = String::new();
        chunks.iter().for_each(|&s| filtered_line.push_str(s));

        let mut muls = Vec::new();
        for mat in re.find_iter(&filtered_line) {
            if let Some(values) = parse_mul(mat.as_str()) {
                muls.push(values);
            }
        }

        total_filtered += compute_sum_of_products(muls);
    }

    Ok((total, total_filtered))
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

    #[test]
    fn parse_sample_test_part2() {
        let data = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let re = Regex::new(r"(?<block>mul\(\d+,\d+\))").unwrap();

        let chunks = get_line_chunks(data);

        let mut filtered_line = String::new();
        chunks.iter().for_each(|&s| filtered_line.push_str(s));

        let mut muls = Vec::new();
        for mat in re.find_iter(&filtered_line) {
            println!("Match: {}", mat.as_str());
            if let Some(values) = parse_mul(mat.as_str()) {
                muls.push(values);
            }
        }

        assert_eq!(muls, vec![(2, 4), (8, 5)]);

        let total = compute_sum_of_products(muls);
        assert_eq!(total, 48);
    }

    #[test]
    fn validate_get_line_chunks() {
        let data = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let chunks = get_line_chunks(data);
        assert_eq!(chunks, vec!["xmul(2,4)&mul[3,7]!^", "?mul(8,5))"]);
    }
}
