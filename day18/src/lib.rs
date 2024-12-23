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

struct MemoryArea {
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),

    corrupted: Vec<(usize, usize)>,
}

impl MemoryArea {
    fn new(
        size: &(usize, usize),
        start: (usize, usize),
        end: (usize, usize),
    ) -> Result<MemoryArea, &'static str> {
        let max_x = cmp::max(start.0, end.0);
        let max_y = cmp::max(start.1, end.1);

        if size.0 > max_x && size.1 > max_y {
            return Ok(MemoryArea {
                max_x,
                max_y,
                start,
                end,

                corrupted: Vec::new(),
            });
        }

        Err("Invalid start, end or size")
    }

    fn set_corrupted(&mut self, corrupted_positions: Vec<(usize, usize)>) {
        self.corrupted = corrupted_positions;
    }

    fn update_corrupted(&mut self, mut corrupted_positions: Vec<(usize, usize)>) {
        self.corrupted.append(&mut corrupted_positions);
    }

    fn compute_shortest_path(&self) {
        todo!();
    }

    fn min_path_len(&self) -> usize {
        todo!();
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

        memory_area.update_corrupted(corrupted);
        memory_area.compute_shortest_path();

        assert_eq!(memory_area.min_path_len(), 22);
    }
}
