use csv::Reader;
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    first_file: String,
    second_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let first_file = args[1].clone();
        let second_file = args[2].clone();

        Ok(Config {
            first_file,
            second_file,
        })
    }
}

struct FileBlock {
    index: usize,
    id_number: u32,
}

struct FreeSpaceBlock {
    index: usize,
}

struct DiskMap {
    file_blocks: Vec<FileBlock>,
    free_space_blocks: Vec<FreeSpaceBlock>,
}

impl DiskMap {
    pub fn make(raw_data: &str) -> Result<DiskMap, &'static str> {
        let mut file_blocks = Vec::new();
        let mut free_space_blocks = Vec::new();
        let mut id_number = 0;
        for (index, c) in raw_data.char_indices() {
            if index % 2 == 0 {
                // file case
                file_blocks.push(FileBlock { index, id_number });
                id_number += 1;
            } else {
                // free block case
                free_space_blocks.push(FreeSpaceBlock { index });
            }
        }

        Ok(DiskMap {
            file_blocks,
            free_space_blocks,
        })
    }

    pub fn to_string(&self) -> String {
        todo!();
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    todo!();
    Ok(())
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn sample_input_validation() {
        let data = "\
2333133121414131402";

        let disk_map = DiskMap::make(data).unwrap();
        assert_eq!(
            disk_map.to_string(),
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );
    }
}
