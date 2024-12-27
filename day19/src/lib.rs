use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fs;
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

fn parse_dictionary_from_raw(raw_dic: &str) -> HashSet<String> {
    raw_dic.split(',').map(|t| t.trim().to_owned()).collect()
}

fn parse_candidates_from_raw(raw_candidates: &str) -> Vec<String> {
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

fn word_split_and_count(s: &str, dictionary: &HashSet<String>) -> u64 {
    let candidate_len = s.len();
    let mut decompositions = vec![Vec::new(); candidate_len + 1];

    decompositions[0].push("".to_owned());

    for ix in 1..(candidate_len + 1) {
        for jx in 0..ix {
            let token = &s[jx..ix];
            if dictionary.contains(token) {
                // jx is always less than ix
                // FIXME
                let (left, right) = decompositions.split_at_mut_checked(jx).unwrap();
                let ix_mapped = ix - jx;

                // FIXME here index jx lead to panic
                for d in left[jx].iter() {
                    right[ix_mapped].push(format!("{}{}", d, token));
                }
            }
        }
    }
    decompositions[candidate_len].len() as u64
}

pub fn run(config: Config) -> Result<(usize, u64), Box<dyn Error>> {
    let dictionary = fs::read_to_string(config.puzzle_input_dictionary)?;
    let combinations = fs::read_to_string(config.puzzle_input_combinations)?;

    let dictionary = parse_dictionary_from_raw(&dictionary);
    let candidates = parse_candidates_from_raw(&combinations);

    let num_possible = candidates
        .iter()
        .filter(|&c| can_split(c, &dictionary))
        .count();

    let total: u64 = candidates
        .iter()
        .map(|c| word_split_and_count(&c, &dictionary))
        .sum();
    Ok((num_possible, total))
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

    #[test]
    fn sample_input_part2_test() {
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

        let mut total = 0;
        for (ix, candidate) in candidates.iter().enumerate() {
            let split = word_split_and_count(&candidate, &dictionary);
            total += split;
        }

        assert_eq!(total, 16);
    }
}
