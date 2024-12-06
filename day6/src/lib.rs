use std::collections::HashSet;
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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
    guard_start_position: (usize, usize),
    guard_start_direction: GuardDirection,
    obstacle: Option<(usize, usize)>,
    position: (usize, usize),
    direction: GuardDirection,
    unique_positions_to_exit: HashSet<(usize, usize)>,
    unique_positions_and_directions: HashSet<((usize, usize), GuardDirection)>,
    loop_found: bool,
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

        let guard_start_position = position;

        let direction = GuardDirection::build(guard).unwrap();
        let guard_start_direction = direction;

        let obstacle = None;

        Ok(LevelMap {
            cells,
            x_max,
            y_max,
            guard_start_position,
            guard_start_direction,
            position,
            obstacle,
            direction,
            unique_positions_to_exit: HashSet::new(),
            unique_positions_and_directions: HashSet::new(),
            loop_found: false,
        })
    }

    fn reset(&mut self) {
        self.position = self.guard_start_position;
        self.direction = self.guard_start_direction;
        self.unique_positions_to_exit.clear();
        self.unique_positions_and_directions.clear();
        self.obstacle = None;
        self.loop_found = false;
    }

    fn set_obstacle(&mut self, obstacle_position: (usize, usize)) {
        self.obstacle = Some(obstacle_position);
    }

    fn move_to_exit(&mut self) {
        while self.move_to_next_cell() {
            if self.loop_found {
                break;
            }
        }
    }

    fn move_to_next_cell(&mut self) -> bool {
        // up to 4 possible cells
        for ix in 0..4 {
            let (delta, next_direction) = match self.direction {
                GuardDirection::Up => ((0, -1), GuardDirection::Right),
                GuardDirection::Right => ((1, 0), GuardDirection::Down),
                GuardDirection::Down => ((0, 1), GuardDirection::Left),
                GuardDirection::Left => ((-1, 0), GuardDirection::Up),
            };

            let next_x: i32 = self.position.0 as i32 + delta.0;
            let next_y: i32 = self.position.1 as i32 + delta.1;
            let next_pos = (next_x, next_y);

            if self.is_cell_inside_map(&next_pos) {
                let next_pos = (next_x as usize, next_y as usize);

                if self.is_cell_free(&next_pos) {
                    self.position = next_pos;
                    self.unique_positions_to_exit.insert(self.position);

                    let pos_and_dir = ((next_pos), next_direction);
                    if self.unique_positions_and_directions.contains(&pos_and_dir) {
                        if let Some(op) = self.obstacle {
                            println!("Found loop for obstacle in position {:?}", op);
                        }
                        self.loop_found = true;
                    } else {
                        self.unique_positions_and_directions.insert(pos_and_dir);
                    }

                    return true;
                }

                self.direction = next_direction;
            } else {
                // the cell is outside the map
                return false;
            }
        }

        // fallback: in this case we tried all possible attempts
        panic!("Attempted all possible directions!");
        false
    }

    fn is_cell_free(&self, position: &(usize, usize)) -> bool {
        assert!(position.0 < self.x_max);
        assert!(position.1 < self.y_max);

        if let Some(obstacle_position) = self.obstacle {
            if obstacle_position == *position {
                return false;
            }
        }

        self.cells[position.1][position.0] != '#'
    }

    fn is_cell_inside_map(&self, position: &(i32, i32)) -> bool {
        position.0 >= 0
            && position.0 < self.x_max as i32
            && position.1 >= 0
            && position.1 < self.y_max as i32
    }

    fn total_unique_positions(&self) -> u32 {
        self.unique_positions_to_exit.len().try_into().unwrap()
    }
}

fn compute_total_unique_positions(raw_data: &str, guard: char) -> u32 {
    let mut map = LevelMap::build(raw_data, guard).unwrap();
    map.move_to_exit();
    map.total_unique_positions()
}

fn compute_total_obstacles_positions(raw_data: &str, guard: char) -> u32 {
    // first get the "critical path"
    let mut map = LevelMap::build(raw_data, guard).unwrap();
    map.move_to_exit();

    let path = map.unique_positions_to_exit.clone();
    let mut num_obstacles: u32 = 0;
    for pos in path {
        map.reset();
        map.set_obstacle(pos);
        map.move_to_exit();

        if map.loop_found {
            num_obstacles += 1;
        }
    }

    num_obstacles
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

    #[test]
    fn part2_logic_test() {
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
        assert_eq!(compute_total_obstacles_positions(data, guard), 6);
    }
}
