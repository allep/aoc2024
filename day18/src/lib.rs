use std::error::Error;

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

struct CorruptedCells {}

impl CorruptedCells {
    fn new(raw_data: &str, size: &(usize, usize)) -> Result<Vec<(usize, usize)>, &'static str> {
        todo!();
    }
}

struct MemoryArea {}

impl MemoryArea {
    fn new(size: &(usize, usize)) -> Result<MemoryArea, &'static str> {
        todo!();
    }

    fn update_corrupted(&mut self, corrupted_positions: Vec<(usize, usize)>) {
        todo!();
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
        let size = (6, 6);
        let corrupted = CorruptedCells::new(raw_data, &size)?;

        let memory_area = MemoryArea::new(&size)?;
        memory_area.update_corrupted(corrupted);
        memory_area.compute_shortest_path();

        assert_eq!(memory_area.min_path_len(), 22);
    }
}
