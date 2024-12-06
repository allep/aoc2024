use std::io::{self, Read};
use std::{error::Error, fs, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input: String,
    guard: char,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let puzzle_input = args[1].clone();
        let guard = match args[2].chars().next() {
            Some(c) if args[2].len() == 1 => c,
            _ => {
                return Err("Invalid second arguments: must be a single char.");
            }
        };

        Ok(Config {
            puzzle_input,
            guard,
        })
    }
}

#[derive(Debug, PartialEq)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

impl GuardDirection {
    fn build(guard_char: char) -> Result<GuardDirection, &'static str> {
        match guard_char {
            '^' => Ok(GuardDirection::Up),
            '>' => Ok(GuardDirection::Right),
            'v' => Ok(GuardDirection::Down),
            '<' => Ok(GuardDirection::Left),
            _ => Err("Invalid guard char"),
        }
    }
}

struct LevelMap {
    cells: Vec<Vec<char>>,
    x_max: usize,
    y_max: usize,
    position: (usize, usize),
    direction: GuardDirection,
    unique_positions_to_exit: Vec<(usize, usize)>,
}

impl LevelMap {
    fn build(raw_data: &str, guard: char) -> Result<LevelMap, &'static str> {
        let lines: Vec<&str> = raw_data.trim().split("\n").collect();
        let y_max = lines.len();
        if y_max == 0 {
            return Err("No lines to parse");
        }

        let x_max = lines[0].len();
        if x_max == 0 {
            return Err("Empty line");
        }

        let cells: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

        // ensure all rows are correct
        let mut cells_ok = true;
        cells.iter().for_each(|l| {
            if l.len() != x_max {
                cells_ok = false;
            }
        });

        if !cells_ok {
            return Err("Variable length lines");
        }

        // search for the start
        let mut position: (usize, usize) = (0, 0);
        cells.iter().enumerate().for_each(|(y, value)| {
            value.iter().enumerate().for_each(|(x, c)| {
                if *c == guard {
                    position = (x, y);
                }
            });
        });

        let direction = GuardDirection::build(guard).unwrap();

        Ok(LevelMap {
            cells,
            x_max,
            y_max,
            position,
            direction,
            unique_positions_to_exit: Vec::new(),
        })
    }

    fn move_to_exit(&self) {
        todo!();
        // while next is inside
    }

    fn total_unique_positions(&self) -> u32 {
        self.unique_positions_to_exit.len().try_into().unwrap()
    }
}

fn compute_total_unique_positions(raw_data: &str, guard: char) -> u32 {
    let map = LevelMap::build(raw_data, guard).unwrap();
    map.move_to_exit();
    map.total_unique_positions()
}

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    let content = fs::read_to_string(config.puzzle_input)?;
    let guard = config.guard;
    let total = compute_total_unique_positions(&content, guard);

    Ok((total))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn part1_level_map_test() {
        let data = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let level_map = LevelMap::build(data, '^').unwrap();
        assert_eq!(level_map.x_max, 10);
        assert_eq!(level_map.y_max, 10);
        assert_eq!(level_map.position, (4, 6));
        assert_eq!(level_map.direction, GuardDirection::Up);
        assert_eq!(level_map.unique_positions_to_exit.len(), 0);
    }

    #[test]
    fn part1_logic_test() {
        let data = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let guard = '^';
        assert_eq!(compute_total_unique_positions(data, guard), 41);
    }
}
