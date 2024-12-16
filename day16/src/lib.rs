use std::collections::HashMap;
use std::collections::HashSet;
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

    fn compute_routing(&mut self) {
        for (iy, row) in self.cells.iter().enumerate() {
            for (ix, _) in row.iter().enumerate() {
                if let Ok(router) = Router::try_make((ix, iy), self) {
                    self.routers.push(router);
                }
            }
        }
    }

    fn is_free_cell(&self, position: (i32, i32)) -> bool {
        assert!(self.is_valid_cell(position));

        let position = (
            usize::try_from(position.0).unwrap(),
            usize::try_from(position.1).unwrap(),
        );

        self.cells[position.1][position.0] != '#'
    }

    fn is_valid_cell(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && usize::try_from(position.0).unwrap() < self.columns
            && position.1 >= 0
            && usize::try_from(position.1).unwrap() < self.rows
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

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Router {
    position: (usize, usize),
    links: HashSet<Direction>,
    metrics: HashMap<Direction, u64>,
}

impl Router {
    pub fn try_make(position: (usize, usize), maze: &Maze) -> Result<Router, &'static str> {
        let pos_int = (
            i32::try_from(position.0).unwrap(),
            i32::try_from(position.1).unwrap(),
        );

        let up = (pos_int.0, pos_int.1 - 1);
        let right = (pos_int.0 + 1, pos_int.1);
        let down = (pos_int.0, pos_int.1 + 1);
        let left = (pos_int.0 - 1, pos_int.1);

        let up = maze.is_valid_cell(up) && maze.is_free_cell(up);
        let right = maze.is_valid_cell(right) && maze.is_free_cell(right);
        let down = maze.is_valid_cell(down) && maze.is_free_cell(down);
        let left = maze.is_valid_cell(left) && maze.is_free_cell(left);

        if (up || down) && (left || right) {
            println!(
                "Created router at ({}, {}) with links:",
                position.0, position.1
            );

            let mut links = HashSet::new();

            if up {
                links.insert(Direction::Up);
                println!(" - Up");
            }

            if right {
                links.insert(Direction::Right);
                println!(" - Right");
            }

            if down {
                links.insert(Direction::Down);
                println!(" - Down");
            }

            if left {
                links.insert(Direction::Left);
                println!(" - Left");
            }

            return Ok(Router {
                position,
                links,
                metrics: HashMap::new(),
            });
        }

        Err("Cell not valid for a router")
    }
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

        let mut maze = Maze::make(data).unwrap();

        assert_eq!(maze.rows(), 15);
        assert_eq!(maze.columns(), 15);
        assert_eq!(maze.position(), (1, 13));

        maze.compute_routing();
    }
}
