use csv::Reader;
use serde::de::DeserializeOwned;
use std::cmp;
use std::error::Error;
use std::io::{self, Read};

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

    corrupted: Vec<(usize, usize)>,
    paths: Vec<Path>,
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

                corrupted: Vec::new(),
                paths: Vec::new(),
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

    fn walk(&mut self, cell: (usize, usize), mut path: Path) {
        path.append(cell).unwrap();
        // self.log_position_and_path(&cell, &path);

        if cell == self.end {
            self.append_start_end_path(path);
            return;
        }

        let next_cells = self.get_next_cells(cell, &path);
        for c in next_cells.iter() {
            self.walk(*c, path.clone());
        }
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

    fn get_non_walked_cells(
        &self,
        mut cells: Vec<(usize, usize)>,
        path: &Path,
    ) -> Vec<(usize, usize)> {
        cells
            .into_iter()
            .filter(|&nc| !path.is_cell_in_path(nc))
            .collect()
    }

    pub fn min_path_len(&self) -> usize {
        let min_num_cells = self.paths.iter().map(|p| p.len()).min().unwrap();
        min_num_cells - 1
    }

    fn get_non_corrupted_cells(&self, cells: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        cells
            .into_iter()
            .filter(|&c| !self.corrupted.contains(&c))
            .collect()
    }
}

pub fn run(config: Config) -> Result<(u64), Box<dyn Error>> {
    // TODO
    Ok((0))
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

        assert_eq!(memory_area.min_path_len(), 22);
    }
}
