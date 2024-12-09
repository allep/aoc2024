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

enum Block {
    FileBlock {
        index: usize,
        num_blocks: u32,
        id_number: u32,
    },
    FreeSpaceBlock {
        index: usize,
        num_blocks: u32,
    },
}

impl Block {
    pub fn to_string(&self) -> String {
        let mut content = String::new();
        match self {
            Block::FileBlock {
                index,
                num_blocks,
                id_number,
            } => {
                let con = format!("{:width$}", id_number, width = *num_blocks as usize);
                content += &con;
            }
            Block::FreeSpaceBlock { index, num_blocks } => {
                let con = format!("{:width$}", ".", width = *num_blocks as usize);
                content += &con;
            }
        }
        content
    }
}

struct DiskMap {
    blocks: Vec<Block>,
}

impl DiskMap {
    pub fn make(raw_data: &str) -> Result<DiskMap, &'static str> {
        let mut blocks = Vec::new();
        let mut id_number = 0;
        for (index, c) in raw_data.char_indices() {
            let num_blocks = c.to_digit(10).unwrap();
            if index % 2 == 0 {
                blocks.push(Block::FileBlock {
                    index,
                    num_blocks,
                    id_number,
                });
                id_number += 1;
            } else {
                blocks.push(Block::FreeSpaceBlock { index, num_blocks });
            }
        }

        Ok(DiskMap { blocks })
    }

    pub fn to_string(&self) -> String {
        let mut representation = String::new();

        for b in &self.blocks {
            representation += &b.to_string();
        }
        representation
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
