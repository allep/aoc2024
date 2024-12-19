use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
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
    routers: HashMap<(usize, usize), Router>,
    dead_ends: Vec<(usize, usize)>,

    already_walked_positions: HashMap<(usize, usize), u64>,
    unique_best_paths_cells: HashSet<(usize, usize)>,
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
                    routers: HashMap::new(),
                    dead_ends: Vec::new(),
                    already_walked_positions: HashMap::new(),
                    unique_best_paths_cells: HashSet::new(),
                });
            }
            _ => (),
        }

        Err("Either start or end positions not found.")
    }

    fn compute_routing(&mut self) {
        for (iy, row) in self.cells.iter().enumerate() {
            for (ix, _) in row.iter().enumerate() {
                let position = (ix, iy);
                if let Ok(router) = Router::try_make(position, self) {
                    self.routers.insert(position, router);
                }

                if self.is_dead_end_cell(position) {
                    self.dead_ends.push(position);
                }
            }
        }

        let ttl: u64 = (self.rows as u64) * (self.columns as u64);
        println!("TTL is {ttl}");
        self.compute_routing_metrics_from_position(self.end, 0, 0, None, ttl);
    }

    fn compute_routing_metrics_from_position(
        &mut self,
        position: (usize, usize),
        current_score: u64,
        current_steps: u64,
        from_direction: Option<Direction>,
        ttl: u64,
    ) {
        if ttl <= 1 {
            return;
        }

        if self.is_returned_to_end_cell(position, from_direction) {
            println!("Returned to end position with score = {current_score}");
            return;
        }

        if self.dead_ends.contains(&position) {
            println!(
                "Found dead end at ({}, {}), returning.",
                position.0, position.1
            );
            return;
        }

        let current_steps = current_steps + 1;

        // basic checks: is end or start?
        if position == self.position {
            match from_direction {
                Some(direction) => {
                    println!(
                        "Found starting position with score = {current_score} and steps = {current_steps} from direction {}",
                        direction.to_str()
                    );
                }
                None => {
                    println!(
                        "Found starting position with score = {current_score} and steps = {current_steps} from direction None"
                    );
                }
            }
            return;
        }

        self.already_walked_positions
            .entry(position)
            .and_modify(|counter| *counter = current_score)
            .or_insert(current_score);

        let mut directions = self.get_valid_free_cells_around(position);

        if let Some(direction) = from_direction {
            match direction {
                Direction::Up => directions.retain(|d| *d != Direction::Down),
                Direction::Right => directions.retain(|d| *d != Direction::Left),
                Direction::Down => directions.retain(|d| *d != Direction::Up),
                Direction::Left => directions.retain(|d| *d != Direction::Right),
            }
        }

        for d in directions.iter() {
            let mut cur_dir_score = current_score;
            if let Some(from_direction) = from_direction {
                if *d != from_direction {
                    cur_dir_score += 1000;
                }
            }

            let mut current = position;
            let mut ttl = ttl;
            while let Some(next) = self.get_next_cell(current, *d) {
                if ttl <= 1 {
                    break;
                }

                current = next;
                cur_dir_score += 1;
                ttl -= 1;

                if self.is_router_cell(current) {
                    let already_walked = match self.routers.get_mut(&current) {
                        Some(router) => {
                            if current == self.position {
                                match d {
                                    Direction::Up => cur_dir_score += 1000,
                                    Direction::Down => cur_dir_score += 1000,
                                    Direction::Right => cur_dir_score += 2000,
                                    _ => (),
                                }
                            };
                            router.update_distance_metric_from_dir(cur_dir_score, current_steps, *d)
                        }
                        _ => false,
                    };

                    if !already_walked {
                        // TODO: not sure if this will have the right values at the end of the day
                        // println!(
                        // "Updating {cur_dir_score} on router in {current:?} from {position:?}"
                        // );

                        self.compute_routing_metrics_from_position(
                            current,
                            cur_dir_score,
                            current_steps,
                            Some(*d),
                            ttl,
                        );
                    }
                }
            }
        }
    }

    fn get_next_cell(
        &self,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let position = (
            i32::try_from(position.0).unwrap(),
            i32::try_from(position.1).unwrap(),
        );

        if !self.is_valid_cell(position) || !self.is_free_cell(position) {
            return None;
        }

        match direction {
            Direction::Up => Some((
                usize::try_from(position.0).unwrap(),
                usize::try_from(position.1 - 1).unwrap(),
            )),
            Direction::Right => Some((
                usize::try_from(position.0 + 1).unwrap(),
                usize::try_from(position.1).unwrap(),
            )),
            Direction::Down => Some((
                usize::try_from(position.0).unwrap(),
                usize::try_from(position.1 + 1).unwrap(),
            )),
            Direction::Left => Some((
                usize::try_from(position.0 - 1).unwrap(),
                usize::try_from(position.1).unwrap(),
            )),
        }
    }

    fn is_returned_to_end_cell(
        &self,
        position: (usize, usize),
        direction: Option<Direction>,
    ) -> bool {
        if let Some(direction) = direction {
            if position == self.end {
                return true;
            }
        }

        false
    }

    fn is_router_cell(&self, position: (usize, usize)) -> bool {
        self.routers.contains_key(&position)
    }

    fn is_free_cell(&self, position: (i32, i32)) -> bool {
        assert!(self.is_valid_cell(position));

        let position = (
            usize::try_from(position.0).unwrap(),
            usize::try_from(position.1).unwrap(),
        );

        self.cells[position.1][position.0] != '#'
    }

    fn is_start_cell(&self, position: (i32, i32)) -> bool {
        let position = (
            usize::try_from(position.0).unwrap(),
            usize::try_from(position.1).unwrap(),
        );
        self.cells[position.1][position.0] == 'S'
    }

    fn is_end_cell(&self, position: (i32, i32)) -> bool {
        let position = (
            usize::try_from(position.0).unwrap(),
            usize::try_from(position.1).unwrap(),
        );
        self.cells[position.1][position.0] == 'E'
    }

    fn is_valid_cell(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && usize::try_from(position.0).unwrap() < self.columns
            && position.1 >= 0
            && usize::try_from(position.1).unwrap() < self.rows
    }

    fn is_dead_end_cell(&self, position: (usize, usize)) -> bool {
        let links = self.get_valid_free_cells_around(position);

        let position = (
            i32::try_from(position.0).unwrap(),
            i32::try_from(position.1).unwrap(),
        );

        self.is_free_cell(position)
            && !self.is_end_cell(position)
            && !self.is_start_cell(position)
            && links.len() > 0
            && links.len() < 2
    }

    pub fn get_valid_free_cells_around(&self, position: (usize, usize)) -> Vec<Direction> {
        let pos_int = (
            i32::try_from(position.0).unwrap(),
            i32::try_from(position.1).unwrap(),
        );

        let up = (pos_int.0, pos_int.1 - 1);
        let right = (pos_int.0 + 1, pos_int.1);
        let down = (pos_int.0, pos_int.1 + 1);
        let left = (pos_int.0 - 1, pos_int.1);

        let up = self.is_valid_cell(up) && self.is_free_cell(up);
        let right = self.is_valid_cell(right) && self.is_free_cell(right);
        let down = self.is_valid_cell(down) && self.is_free_cell(down);
        let left = self.is_valid_cell(left) && self.is_free_cell(left);

        let mut links = Vec::new();

        if up {
            links.push(Direction::Up);
        }

        if right {
            links.push(Direction::Right);
        }

        if down {
            links.push(Direction::Down);
        }

        if left {
            links.push(Direction::Left);
        }

        links
    }

    pub fn get_min_score(&self) -> u64 {
        if let Some(router) = self.routers.get(&self.position) {
            let mut scores = Vec::new();

            for (dir, metrics) in router.metrics.iter() {
                scores.push(metrics.0);
            }

            if let Some(min) = scores.iter().min() {
                return *min;
            }
        }

        0
    }

    pub fn walk_through_best_paths(&mut self) {
        let mut min_metric = 0;
        let mut min_directions: HashMap<u64, Vec<Direction>> = HashMap::new();
        if let Some(start_router) = self.routers.get(&self.position) {
            for (d, metrics) in start_router.metrics.iter() {
                if min_metric == 0 {
                    min_metric = metrics.1;
                }

                if metrics.1 < min_metric {
                    min_metric = metrics.1;
                }

                min_directions
                    .entry(metrics.1)
                    .and_modify(|dir| dir.push(*d))
                    .or_insert(vec![*d]);
            }
        }

        println!("Starting from {:?}", self.position);
        self.unique_best_paths_cells.insert(self.position);

        // now actually walk through using always the same lowest metrics
        if let Some(min_directions) = min_directions.get(&min_metric) {
            for d in min_directions {
                self.walk_to_router_or_end(self.position, *d);
            }
        }
    }

    fn walk_to_router_or_end(&mut self, from: (usize, usize), direction: Direction) {
        let mut current = from;
        while let Some(next) = self.get_next_cell(current, direction) {
            current = next;
            self.unique_best_paths_cells.insert(current);
            println!(" - Walking on {:?}", current);

            if self.end == current {
                return;
            }

            if self.is_router_cell(current) {
                let mut min_metric = 0;
                let mut min_directions: HashMap<u64, Vec<Direction>> = HashMap::new();
                if let Some(router) = self.routers.get(&current) {
                    for (dir, metrics) in router.metrics.iter() {
                        println!(
                            "    - Found direction {} with metric {}",
                            dir.to_str(),
                            metrics.1
                        );
                        if min_metric == 0 {
                            min_metric = metrics.1;
                        }

                        if metrics.1 < min_metric {
                            min_metric = metrics.1;
                        }

                        min_directions
                            .entry(metrics.1)
                            .and_modify(|d| d.push(*dir))
                            .or_insert(vec![*dir]);
                    }
                }

                if let Some(min_directions) = min_directions.get(&min_metric) {
                    println!("   - Found n directions: {}", min_directions.len());
                    for d in min_directions {
                        self.walk_to_router_or_end(current, *d);
                    }
                }

                break;
            }
        }
    }

    pub fn count_unique_best_paths_cells(&self) -> u64 {
        self.unique_best_paths_cells.len() as u64
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

    pub fn dead_cells(&self) -> Vec<(usize, usize)> {
        self.dead_ends.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::Up => "Up",
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
        }
    }
}

struct Router {
    position: (usize, usize),
    links: HashSet<Direction>,
    metrics: HashMap<Direction, (u64, u64)>,
}

impl Router {
    pub fn try_make(position: (usize, usize), maze: &Maze) -> Result<Router, &'static str> {
        let free_valid_positions = maze.get_valid_free_cells_around(position);

        let up = free_valid_positions.contains(&Direction::Up);
        let down = free_valid_positions.contains(&Direction::Down);
        let left = free_valid_positions.contains(&Direction::Left);
        let right = free_valid_positions.contains(&Direction::Right);

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

    pub fn update_distance_metric_from_dir(
        &mut self,
        score: u64,
        steps: u64,
        from_direction: Direction,
    ) -> bool {
        let mut already_walked = false;
        match from_direction {
            Direction::Up => self
                .metrics
                .entry(Direction::Down)
                .and_modify(|c| {
                    if c.0 > score {
                        c.0 = score;
                        c.1 = steps;
                    } else {
                        already_walked = true;
                    }
                })
                .or_insert((score, steps)),
            Direction::Right => self
                .metrics
                .entry(Direction::Left)
                .and_modify(|c| {
                    if c.0 > score {
                        c.0 = score;
                        c.1 = steps;
                    } else {
                        already_walked = true;
                    }
                })
                .or_insert((score, steps)),
            Direction::Down => self
                .metrics
                .entry(Direction::Up)
                .and_modify(|c| {
                    if c.0 > score {
                        c.0 = score;
                        c.1 = steps;
                    } else {
                        already_walked = true;
                    }
                })
                .or_insert((score, steps)),
            Direction::Left => self
                .metrics
                .entry(Direction::Right)
                .and_modify(|c| {
                    if c.0 > score {
                        c.0 = score;
                        c.1 = steps;
                    } else {
                        already_walked = true;
                    }
                })
                .or_insert((score, steps)),
        };

        return already_walked;
    }
}

pub fn run(config: Config) -> Result<(u64, u64), Box<dyn Error>> {
    let raw_content = fs::read_to_string(config.puzzle_input)?;
    let mut maze = Maze::make(&raw_content).unwrap();

    maze.compute_routing();
    let score = maze.get_min_score();
    maze.walk_through_best_paths();
    let unique = maze.count_unique_best_paths_cells();

    Ok((score, unique))
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
        // assert_eq!(maze.get_min_score(), 7036);

        maze.walk_through_best_paths();
        let unique = maze.count_unique_best_paths_cells();
        assert_eq!(unique, 45);
    }
}
