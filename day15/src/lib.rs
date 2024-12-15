use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input_map: String,
    puzzle_input_moves: String,
}

struct Moves {
    moves: Vec<char>,
}

impl Moves {
    pub fn make(raw_data: &str) -> Result<Moves, &'static str> {
        let moves: Vec<char> = raw_data
            .trim()
            .as_bytes()
            .iter()
            .map(|b| *b as char)
            .filter(|c| *c as char != '\n')
            .collect();

        let mut is_ok = true;
        moves.iter().for_each(|c| {
            if *c != '^' && *c != 'v' && *c != '<' && *c != '>' {
                is_ok = false;
            }
        });

        if !is_ok {
            return Err("Some moves were not acceptable");
        }

        Ok(Moves { moves })
    }
}

struct WarehouseMap {
    positions: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl WarehouseMap {
    pub fn make(raw_data: &str) -> Result<WarehouseMap, &'static str> {
        let lines: Vec<String> = raw_data.trim().split("\n").map(|s| s.to_string()).collect();
        let num_rows = lines.len();
        if num_rows == 0 {
            return Err("No row deserialized");
        }

        let mut positions = Vec::new();
        let mut is_ok = true;
        let mut num_columns = 0;
        for l in lines {
            let chars: Vec<char> = l.chars().into_iter().collect();

            let length = chars.len();
            if num_columns == 0 {
                num_columns = length;
            } else {
                if length != num_columns {
                    is_ok = false;
                }
            }

            positions.push(chars);
        }

        if !is_ok {
            return Err("Wrong line length for map.");
        }

        Ok(WarehouseMap {
            positions,
            rows: num_rows,
            columns: num_columns,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let puzzle_input_map = args[1].clone();
        let puzzle_input_moves = args[2].clone();

        Ok(Config {
            puzzle_input_map,
            puzzle_input_moves,
        })
    }
}

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    Ok((0))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn basic_moves_creation_test() {
        let data = "\
<^^>>>vv<v>>v<<";

        let m = Moves::make(data).unwrap();
        assert_eq!(m.moves.len(), 15);
    }

    #[test]
    fn basic_map_creation_test() {
        let data = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let map = WarehouseMap::make(data).unwrap();
        assert_eq!(map.rows(), 8);
        assert_eq!(map.columns(), 8);
    }
}
