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

struct Block {
    id_number: Option<u32>,
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
                for ix in 0..num_blocks {
                    blocks.push(Block {
                        id_number: Some(id_number),
                    });
                }

                id_number += 1;
            } else {
                for ix in 0..num_blocks {
                    blocks.push(Block { id_number: None });
                }
            }
        }

        Ok(DiskMap { blocks })
    }

    pub fn to_string(&self) -> String {
        let mut representation = String::new();

        for b in &self.blocks {
            match b.id_number {
                Some(id_number) => {
                    representation += &format!("{}", id_number);
                }

                None => {
                    representation += ".";
                }
            }
        }
        representation
    }

    pub fn defrag(&mut self) {
        let empty_blocks_indices: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .filter(|&(_, value)| value.id_number.is_none())
            .map(|(index, _)| index)
            .collect();

        let mut file_id_to_defrag = Vec::new();
        let mut to_defrag_indexes = Vec::new();
        for (index, b) in self.blocks.iter().rev().enumerate() {
            if let Some(file_id) = b.id_number {
                file_id_to_defrag.push(file_id);
                to_defrag_indexes.push(index);
            }
        }

        let mut to_defrag_iter = file_id_to_defrag.iter();
        let mut to_defrag_index_iter = to_defrag_indexes.iter();

        for ix in empty_blocks_indices {
            let next = to_defrag_iter.next();
            let next_index = to_defrag_index_iter.next();

            if let (Some(file_id), Some(file_id_index)) = (next, next_index) {
                self.blocks[ix].id_number = Some(*file_id);
                self.blocks[*file_id_index].id_number = None;
            }
        }
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

        let mut disk_map = DiskMap::make(data).unwrap();
        assert_eq!(
            disk_map.to_string(),
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );

        disk_map.defrag();

        assert_eq!(
            disk_map.to_string(),
            String::from("0099811188827773336446555566.............")
        );
    }
}
