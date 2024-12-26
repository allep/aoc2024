use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input_dictionary: String,
    puzzle_input_combinations: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let puzzle_input_dictionary = args[1].clone();
        let puzzle_input_combinations = args[2].clone();

        Ok(Config {
            puzzle_input_dictionary,
            puzzle_input_combinations,
        })
    }
}

pub fn parse_dictionary_from_raw(raw_dic: &str) -> HashSet<String> {
    raw_dic.split(',').map(|t| t.trim().to_owned()).collect()
}

pub fn parse_candidates_from_raw(raw_candidates: &str) -> Vec<String> {
    raw_candidates.lines().map(|s| s.to_owned()).collect()
}

fn can_split(s: &str, dictionary: &HashSet<String>) -> bool {
    let candidate_len = s.len();
    let mut can_be_split = vec![false; candidate_len + 1];

    can_be_split[0] = true;

    for ix in 1..(candidate_len + 1) {
        for jx in 0..ix {
            if can_be_split[jx] && dictionary.contains(&s[jx..ix]) {
                can_be_split[ix] = true;
                break;
            }
        }
    }

    can_be_split[candidate_len]
}

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    // TODO
    Ok(0)
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn sample_input_test() {
        let dictionary = "\
r, wr, b, g, bwu, rb, gb, br";
        let candidates = "\
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let expected = vec![true, true, true, true, false, true, true, false];

        let dictionary = parse_dictionary_from_raw(dictionary);
        assert_eq!(dictionary.len(), 8);
        let candidates = parse_candidates_from_raw(candidates);
        assert_eq!(candidates.len(), 8);
        assert_eq!(candidates.len(), expected.len());

        for (ix, candidate) in candidates.iter().enumerate() {
            assert_eq!(can_split(&candidate, &dictionary), expected[ix]);
        }
    }
}
