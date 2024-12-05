use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
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

#[derive(Debug)]
struct WordSearch {
    lines: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct CandidateWord {
    letters: Vec<(usize, usize)>,
}

fn get_lines(raw_input: &str) -> Vec<&str> {
    let chunks: Vec<&str> = raw_input.trim().split("\n").collect();
    chunks
}

impl WordSearch {
    fn build(raw_content: &str) -> Result<WordSearch, &'static str> {
        let lines: Vec<String> = raw_content
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        Ok(WordSearch { lines })
    }

    fn compute(&self, word: &str) -> u32 {
        let key_letters = word.as_bytes();
        let c = key_letters[0] as char;
        let positions = self.get_positions(c);
        for pos in &positions {
            println!("Position found for char {c}: ({}, {})", pos.0, pos.1);
        }

        let candidates = self.get_candidates(positions, word);

        for c in &candidates {
            println!("Found candidate: {c:?}");
        }

        candidates.len().try_into().unwrap()
    }

    fn get_positions(&self, letter: char) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        self.lines
            .iter()
            .enumerate()
            .for_each(|(line_number, &ref line)| {
                line.match_indices(letter)
                    .for_each(|(column, &ref c)| positions.push((line_number, column)));
            });

        positions
    }

    fn get_candidates(&self, positions: Vec<(usize, usize)>, word: &str) -> HashSet<CandidateWord> {
        let key_letters = word.as_bytes();
        let length = word.len();
        let num_lines = self.lines.len();

        assert!(num_lines > 0);

        let line_length = self.lines[0].len();

        assert!(line_length > 0);

        let max_decrement = length - 1;

        let mut candidates = HashSet::new();
        for pos in positions {
            // we have up to 8 candidates

            // upper vertical
            if pos.0 >= max_decrement {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 - ix, pos.1);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // first diagonal
            if pos.0 >= max_decrement && pos.1 <= line_length - length {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 - ix, pos.1 + ix);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // right horizontal
            if pos.1 <= line_length - length {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0, pos.1 + ix);
                    let c = key_letters[ix] as char;
                    let r = self.get_letter(p);
                    if c == r {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // second diagonal
            if pos.0 <= num_lines - length && pos.1 <= line_length - length {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 + ix, pos.1 + ix);
                    let c = key_letters[ix] as char;
                    let r = self.get_letter(p);
                    if c == r {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // down vertical
            if pos.0 <= num_lines - length {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 + ix, pos.1);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // third diagonal
            if pos.0 <= num_lines - length && pos.1 >= max_decrement {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 + ix, pos.1 - ix);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // left horizontal
            if pos.1 >= max_decrement {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0, pos.1 - ix);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }

            // fourth diagonal
            if pos.0 >= max_decrement && pos.1 >= max_decrement {
                let mut letter_pos = Vec::new();
                let mut success = true;
                for ix in 0..length {
                    let p = (pos.0 - ix, pos.1 - ix);
                    let c = key_letters[ix] as char;
                    if c == self.get_letter(p) {
                        letter_pos.push(p);
                    } else {
                        success = false;
                        break;
                    }
                }

                if success {
                    let candidate = CandidateWord {
                        letters: letter_pos,
                    };

                    candidates.insert(candidate);
                }
            }
        }

        candidates
    }

    fn get_letter(&self, position: (usize, usize)) -> char {
        let num_lines = self.lines.len();
        let line_length = self.lines[0].len();
        assert!(position.0 < num_lines, "x = {}", position.0);
        assert!(position.1 < line_length, "y = {}", position.1);

        self.lines[position.0].as_bytes()[position.1] as char
    }
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

fn compute_total_xmas(raw_data: &str) -> u32 {
    let word_search = WordSearch::build(raw_data).unwrap();
    word_search.compute("XMAS")
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
    fn part1_simplified_logic_test() {
        let data = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!(compute_total_xmas(data), 4);
    }

    #[test]
    fn part1_logic_test() {
        let data = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(compute_total_xmas(data), 18);
    }
}
