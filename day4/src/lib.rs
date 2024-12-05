use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::collections::HashSet;
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
        let candidates = self.get_candidates(positions, word);

        candidates.len().try_into().unwrap()
    }

    fn compute_cross_mas(&self, word: &str) -> u32 {
        let key_letters = word.as_bytes();
        let c = key_letters[0] as char;
        let positions = self.get_positions(c);
        let candidates = self.get_cross_candidates(positions, word);

        let cross_total = self.compute_cross_total_from_candidates(candidates);
        cross_total
    }

    fn compute_cross_total_from_candidates(&self, candidates: HashSet<CandidateWord>) -> u32 {
        let mut cross_candidates = HashMap::new();

        for c in &candidates {
            assert_eq!(c.letters.len(), 3);

            cross_candidates
                .entry(c.letters[1])
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut total: u32 = 0;
        for c in &cross_candidates {
            assert!(*c.1 <= 2);

            if *c.1 == 2 {
                total += 1;
            }
        }

        total
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

    fn get_cross_candidates(
        &self,
        positions: Vec<(usize, usize)>,
        word: &str,
    ) -> HashSet<CandidateWord> {
        let key_letters = word.as_bytes();
        let length = word.len();
        let num_lines = self.lines.len();

        assert!(num_lines > 0);

        let line_length = self.lines[0].len();

        assert!(line_length > 0);

        let max_decrement = length - 1;

        let mut candidates = HashSet::new();
        for pos in positions {
            // we have up to 4 candidates

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

fn compute_total_xmas_part2(raw_data: &str) -> u32 {
    let word_search = WordSearch::build(raw_data).unwrap();
    word_search.compute_cross_mas("MAS")
}

pub fn run(config: Config) -> Result<(u32, u32), Box<dyn Error>> {
    let content = fs::read_to_string(config.puzzle_input)?;
    let total = compute_total_xmas(&content);
    let total_cross_mas = compute_total_xmas_part2(&content);

    Ok((total, total_cross_mas))
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

    #[test]
    fn part2_logic_test() {
        let data = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!(compute_total_xmas_part2(data), 9);
    }
}
