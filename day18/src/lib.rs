use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read};
use std::rc::Rc;
use std::{cmp, fs};

#[derive(Debug)]
pub struct Config {
    puzzle_input: String,
    max_length: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments").into());
        }

        let puzzle_input = args[1].clone();
        let max_length = args[2].parse::<usize>()?;

        Ok(Config {
            puzzle_input,
            max_length,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
struct CorruptedCell {
    x: usize,
    y: usize,
}

fn build_corrupted_cells<T, R>(reader: R, size: &(usize, usize)) -> Result<Vec<T>, Box<dyn Error>>
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

enum BFSCell<'a> {
    Child {
        position: (usize, usize),
        parent: Rc<BFSCell<'a>>,
        map_size: (usize, usize),
        corrupted_positions: &'a [(usize, usize)],
    },
    Root {
        position: (usize, usize),
        map_size: (usize, usize),
        corrupted_positions: &'a [(usize, usize)],
    },
}

impl<'a> BFSCell<'a> {
    pub fn new_root(
        position: (usize, usize),
        map_size: (usize, usize),
        corrupted_positions: &'a [(usize, usize)],
    ) -> BFSCell<'a> {
        BFSCell::Root {
            position,
            map_size,
            corrupted_positions,
        }
    }

    pub fn new_child(
        position: (usize, usize),
        parent: Rc<BFSCell<'a>>,
        map_size: (usize, usize),
        corrupted_positions: &'a [(usize, usize)],
    ) -> BFSCell<'a> {
        BFSCell::Child {
            position,
            parent,
            map_size,
            corrupted_positions,
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        match *self {
            BFSCell::Root { position, .. } | BFSCell::Child { position, .. } => position,
        }
    }

    pub fn get_next_cells(&self) -> Vec<(usize, usize)> {
        let debug = false;
        let next_cells = self.get_next_cells_inside_map();
        if debug {
            next_cells
                .iter()
                .for_each(|&nc| println!("Next possible cell 1: ({}, {})", nc.0, nc.1));
        }

        let next_cells = self.get_non_corrupted_cells(next_cells);
        if debug {
            next_cells
                .iter()
                .for_each(|&nc| println!("Next possible cell 2: ({}, {})", nc.0, nc.1));
        }

        next_cells
    }

    fn get_next_cells_inside_map(&self) -> Vec<(usize, usize)> {
        match *self {
            BFSCell::Root {
                position, map_size, ..
            }
            | BFSCell::Child {
                position, map_size, ..
            } => {
                let x = i64::try_from(position.0).unwrap();
                let y = i64::try_from(position.1).unwrap();
                let max_x = i64::try_from(map_size.0).unwrap();
                let max_y = i64::try_from(map_size.1).unwrap();

                let mut next_cells = Vec::new();
                if x - 1 >= 0 {
                    next_cells.push((position.0 - 1, position.1));
                }

                if y - 1 >= 0 {
                    next_cells.push((position.0, position.1 - 1));
                }

                if x + 1 < max_x {
                    next_cells.push((position.0 + 1, position.1));
                }

                if y + 1 < max_y {
                    next_cells.push((position.0, position.1 + 1));
                }

                return next_cells;
            }
        }
    }

    fn get_non_corrupted_cells(&self, cells: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        match *self {
            BFSCell::Root {
                position,
                corrupted_positions,
                ..
            }
            | BFSCell::Child {
                position,
                corrupted_positions,
                ..
            } => cells
                .into_iter()
                .filter(|&c| !corrupted_positions.contains(&c))
                .collect(),
        }
    }
}

#[derive(Clone)]
struct Path {
    max_x: usize,
    max_y: usize,
    cells: Vec<(usize, usize)>,
}

impl Path {
    pub fn new(size: (usize, usize)) -> Result<Path, &'static str> {
        Ok(Path {
            max_x: size.0,
            max_y: size.1,
            cells: Vec::new(),
        })
    }

    pub fn append(&mut self, cell: (usize, usize)) -> Result<(), &'static str> {
        if cell.0 < self.max_x && cell.1 < self.max_y {
            self.cells.push(cell);
            return Ok(());
        }

        Err("Invalid position within path.")
    }

    pub fn is_cell_in_path(&self, cell: (usize, usize)) -> bool {
        self.cells.contains(&cell)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

struct MemoryArea {
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),
    max_length: usize,

    corrupted: Vec<(usize, usize)>,
    paths: Vec<Path>,
    walked_cells: u64,
    min_steps_bfs: usize,
    goal_found: bool,
}

impl MemoryArea {
    pub fn new(
        size: &(usize, usize),
        start: (usize, usize),
        end: (usize, usize),
    ) -> Result<MemoryArea, &'static str> {
        let max_x = cmp::max(start.0, end.0);
        let max_y = cmp::max(start.1, end.1);

        if size.0 > max_x && size.1 > max_y {
            let max_x = size.0;
            let max_y = size.1;
            return Ok(MemoryArea {
                max_x,
                max_y,
                start,
                end,
                max_length: 0,

                corrupted: Vec::new(),
                paths: Vec::new(),
                walked_cells: 0,
                min_steps_bfs: 0,
                goal_found: false,
            });
        }

        Err("Invalid start, end or size")
    }

    pub fn set_corrupted(&mut self, corrupted_positions: &[(usize, usize)]) {
        self.corrupted = corrupted_positions.to_vec();
    }

    pub fn update_corrupted(&mut self, corrupted_positions: &[(usize, usize)]) {
        self.corrupted.append(&mut corrupted_positions.to_vec());
    }

    pub fn num_corrupted(&self) -> usize {
        self.corrupted.len()
    }

    pub fn compute_paths(&mut self) {
        let size = (self.max_x, self.max_y);
        self.walk(self.start, Path::new(size).unwrap());
    }

    pub fn compute_paths_bfs(&mut self) {
        self.breadth_first_search(self.start);
    }

    fn breadth_first_search(&mut self, root_position: (usize, usize)) {
        let mut queue: VecDeque<Rc<BFSCell>> = VecDeque::new();
        let root = Rc::new(BFSCell::new_root(
            root_position,
            (self.max_x, self.max_y),
            &self.corrupted,
        ));
        queue.push_back(root);

        let mut explored = HashSet::new();
        explored.insert(root_position);

        self.goal_found = false;
        while let Some(current) = queue.pop_front() {
            if self.is_goal_reached(&current) {
                let end_pos = current.get_position();
                let steps = Self::compute_min_steps_bfs(&current);
                self.min_steps_bfs = steps;
                self.goal_found = true;

                println!(
                    "Goal reached on position ({}, {}), with steps = {}",
                    end_pos.0, end_pos.1, steps
                );
                return;
            }

            let next_cells = current.get_next_cells();
            for c in next_cells.iter() {
                if !explored.contains(c) {
                    explored.insert(*c);
                    let to_explore = Rc::new(BFSCell::new_child(
                        *c,
                        Rc::clone(&current),
                        (self.max_x, self.max_y),
                        &self.corrupted,
                    ));
                    queue.push_back(to_explore);
                }
            }
        }
    }

    fn goal_found(&self) -> bool {
        self.goal_found
    }

    fn is_goal_reached(&self, cell: &Rc<BFSCell>) -> bool {
        let is_end = match **cell {
            BFSCell::Child { position, .. } | BFSCell::Root { position, .. } => {
                position == self.end
            }
        };
        is_end
    }

    fn compute_min_steps_bfs(cell: &Rc<BFSCell>) -> usize {
        let mut cell_count: usize = 0;

        let mut next = cell;
        loop {
            match **next {
                BFSCell::Root { .. } => {
                    cell_count += 1;
                    break;
                }
                BFSCell::Child {
                    position,
                    ref parent,
                    ..
                } => {
                    cell_count += 1;
                    next = parent;
                }
            }
        }

        cell_count - 1
    }

    fn walk(&mut self, cell: (usize, usize), mut path: Path) {
        self.walked_cells += 1;
        path.append(cell).unwrap();

        if self.walked_cells % 1000000 == 0 {
            println!("Walked 1M cells");
            self.log_position_and_path(&cell, &path);
        }

        if cell == self.end {
            self.append_start_end_path(path);
            return;
        }

        if self.is_path_too_long(&path) {
            return;
        }

        let next_cells = self.get_next_cells(cell, &path);
        for c in next_cells.iter() {
            self.walk(*c, path.clone());
        }
    }

    fn is_path_too_long(&self, path: &Path) -> bool {
        path.len() >= self.max_length
    }

    fn append_start_end_path(&mut self, path: Path) {
        println!("Found start-end path with len = {}", path.len());
        self.paths.push(path);
    }

    fn log_position_and_path(&self, cell: &(usize, usize), path: &Path) {
        println!(
            "Walking on ({}, {}), path len is {}",
            cell.0,
            cell.1,
            path.len()
        );
    }

    fn get_next_cells_from_root(&self, root: &(usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn get_next_cells_from_child(
        &self,
        root: &(usize, usize),
        parent: &(usize, usize),
    ) -> Vec<(usize, usize)> {
        todo!()
    }

    fn get_next_cells(&self, cell: (usize, usize), path: &Path) -> Vec<(usize, usize)> {
        let debug = false;
        let next_cells = self.get_next_cells_inside_map(cell);
        if debug {
            next_cells
                .iter()
                .for_each(|&nc| println!("Next possible cell 1: ({}, {})", nc.0, nc.1));
        }

        let next_cells = self.get_non_walked_cells(next_cells, &path);
        if debug {
            next_cells
                .iter()
                .for_each(|&nc| println!("Next possible cell 2: ({}, {})", nc.0, nc.1));
        }

        let next_cells = self.get_non_corrupted_cells(next_cells);
        if debug {
            next_cells
                .iter()
                .for_each(|&nc| println!("Next possible cell 3: ({}, {})", nc.0, nc.1));
        }

        next_cells
    }

    fn get_next_cells_inside_map(&self, cell: (usize, usize)) -> Vec<(usize, usize)> {
        let x = i64::try_from(cell.0).unwrap();
        let y = i64::try_from(cell.1).unwrap();
        let max_x = i64::try_from(self.max_x).unwrap();
        let max_y = i64::try_from(self.max_y).unwrap();

        let mut next_cells = Vec::new();
        if x - 1 >= 0 {
            next_cells.push((cell.0 - 1, cell.1));
        }

        if y - 1 >= 0 {
            next_cells.push((cell.0, cell.1 - 1));
        }

        if x + 1 < max_x {
            next_cells.push((cell.0 + 1, cell.1));
        }

        if y + 1 < max_y {
            next_cells.push((cell.0, cell.1 + 1));
        }

        next_cells
    }

    fn get_non_walked_cells(&self, cells: Vec<(usize, usize)>, path: &Path) -> Vec<(usize, usize)> {
        cells
            .into_iter()
            .filter(|&nc| !path.is_cell_in_path(nc))
            .collect()
    }

    pub fn min_steps_bfs(&self) -> Result<usize, &'static str> {
        if self.min_steps_bfs != 0 {
            return Ok(self.min_steps_bfs);
        }

        Err("No path found")
    }

    pub fn min_path_len(&self) -> Result<usize, &'static str> {
        if !self.paths.is_empty() {
            let min_num_cells = self.paths.iter().map(|p| p.len()).min().unwrap();
            return Ok(min_num_cells - 1);
        }
        Err("No path found")
    }

    fn get_non_corrupted_cells(&self, cells: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        cells
            .into_iter()
            .filter(|&c| !self.corrupted.contains(&c))
            .collect()
    }
}

pub fn run(config: Config) -> Result<(usize, usize), Box<dyn Error>> {
    let raw_data = fs::read_to_string(config.puzzle_input)?;
    let size = (71, 71);
    let corrupted: Vec<CorruptedCell> = build_corrupted_cells(raw_data.as_bytes(), &size).unwrap();
    let corrupted: Vec<(usize, usize)> = corrupted.iter().map(|c| (c.x, c.y)).collect();

    let mut corrupted_pos = None;
    for ix in 0..corrupted.len() {
        println!("Simulating index {} ...", ix);

        let start = (0, 0);
        let end = (70, 70);
        let mut memory_area = MemoryArea::new(&size, start, end).unwrap();
        let num_corrupted = ix + 1;
        memory_area.set_corrupted(&corrupted[0..num_corrupted]);
        memory_area.compute_paths_bfs();

        if !memory_area.goal_found() {
            corrupted_pos = Some(corrupted[ix]);
            break;
        }
    }

    if let Some(pos) = corrupted_pos {
        return Ok(pos);
    }

    return Err(String::from("Didn't find any position making the maze impossible").into());
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn sample_input_path_length_compute_test() {
        let raw_data = "\
x,y
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let size = (7, 7);

        let corrupted: Vec<CorruptedCell> =
            build_corrupted_cells(raw_data.as_bytes(), &size).unwrap();
        let corrupted: Vec<(usize, usize)> = corrupted.iter().map(|c| (c.x, c.y)).collect();
        assert_eq!(corrupted.len(), 25);

        let start = (0, 0);
        let end = (6, 6);
        let mut memory_area = MemoryArea::new(&size, start, end).unwrap();

        assert_eq!(memory_area.num_corrupted(), 0);
        memory_area.set_corrupted(&corrupted[0..12]);
        assert_eq!(memory_area.num_corrupted(), 12);

        memory_area.compute_paths();

        assert_eq!(memory_area.min_path_len().unwrap(), 22);
    }

    #[test]
    fn sample_input_path_length_compute_bfs_test() {
        let raw_data = "\
x,y
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let size = (7, 7);

        let corrupted: Vec<CorruptedCell> =
            build_corrupted_cells(raw_data.as_bytes(), &size).unwrap();
        let corrupted: Vec<(usize, usize)> = corrupted.iter().map(|c| (c.x, c.y)).collect();
        assert_eq!(corrupted.len(), 25);

        let start = (0, 0);
        let end = (6, 6);
        let mut memory_area = MemoryArea::new(&size, start, end).unwrap();

        assert_eq!(memory_area.num_corrupted(), 0);
        memory_area.set_corrupted(&corrupted[0..12]);
        assert_eq!(memory_area.num_corrupted(), 12);

        memory_area.compute_paths_bfs();

        assert_eq!(memory_area.min_steps_bfs().unwrap(), 22);
    }
    #[test]
    fn sample_input_part2_bfs_test() {
        let raw_data = "\
x,y
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let expected_goal_found: [bool; 25] = [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, false, false, false, false, false,
        ];

        let size = (7, 7);

        let corrupted: Vec<CorruptedCell> =
            build_corrupted_cells(raw_data.as_bytes(), &size).unwrap();
        let corrupted: Vec<(usize, usize)> = corrupted.iter().map(|c| (c.x, c.y)).collect();
        assert_eq!(corrupted.len(), 25);

        for ix in 0..corrupted.len() {
            let start = (0, 0);
            let end = (6, 6);
            let mut memory_area = MemoryArea::new(&size, start, end).unwrap();
            let num_corrupted = ix + 1;
            memory_area.set_corrupted(&corrupted[0..num_corrupted]);
            assert_eq!(memory_area.num_corrupted(), num_corrupted);
            memory_area.compute_paths_bfs();
            assert_eq!(
                memory_area.goal_found(),
                expected_goal_found[ix],
                "Expected goal for index {} comparison failed",
                ix
            );
        }
    }
}
