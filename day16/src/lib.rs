use std::collections::HashMap;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

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

struct Maze {
    cells: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    position: (usize, usize),
    end: (usize, usize),
    routers: Vec<Router>,
}

impl Maze {
    pub fn make(raw_data: &str) -> Result<Maze, &'static str> {
        let lines: Vec<String> = raw_data.trim().split("\n").map(|s| s.to_string()).collect();
        let num_rows = lines.len();
        if num_rows == 0 {
            return Err("No row deserialized");
        }

        let mut cells = Vec::new();
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

            cells.push(chars);
        }

        if !is_ok {
            return Err("Wrong line length for map.");
        }

        let mut start_position = None;
        let mut end_position = None;
        for (iy, row) in cells.iter().enumerate() {
            for (ix, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start_position = Some((ix, iy));
                }

                if *c == 'E' {
                    end_position = Some((ix, iy));
                }
            }
        }

        match (start_position, end_position) {
            (Some(start), Some(end)) => {
                return Ok(Maze {
                    cells,
                    rows: num_rows,
                    columns: num_columns,
                    position: start,
                    end,
                    routers: Vec::new(),
                });
            }
            _ => (),
        }

        Err("Either start or end positions not found.")
    }

    fn is_position_valid(position: (i32, i32), num_rows: usize, num_columns: usize) -> bool {
        position.0 >= 0
            && usize::try_from(position.0).unwrap() < num_columns
            && position.1 >= 0
            && usize::try_from(position.1).unwrap() < num_rows
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Router {
    position: (usize, usize),
    metrics: HashMap<Direction, u64>,
}

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    // TODO
    Ok(0)
}

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn sample_input_test() {
        let data = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let maze = Maze::make(data).unwrap();

        assert_eq!(maze.rows(), 15);
        assert_eq!(maze.columns(), 15);
        assert_eq!(maze.position(), (1, 13));
    }
}
