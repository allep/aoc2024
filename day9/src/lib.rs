use csv::Reader;
use serde::de::DeserializeOwned;
use std::fs;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    first_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let first_file = args[1].clone();

        Ok(Config { first_file })
    }
}

struct Block {
    id_number: Option<u32>,
}

struct FreeBlockInfo {
    starting_index: usize,
    size: usize,
}

struct DiskMap {
    blocks: Vec<Block>,
    free_blocks_cache: Option<Vec<FreeBlockInfo>>,
}

impl DiskMap {
    pub fn make(raw_data: &str) -> Result<DiskMap, &'static str> {
        let mut blocks = Vec::new();
        let mut free_blocks = Vec::new();
        let mut id_number = 0;
        let mut running_starting_index: u32 = 0;
        for (index, c) in raw_data.char_indices() {
            if let Some(num_blocks) = c.to_digit(10) {
                if index % 2 == 0 {
                    for ix in 0..num_blocks {
                        blocks.push(Block {
                            id_number: Some(id_number),
                        });
                    }

                    id_number += 1;
                } else {
                    free_blocks.push(FreeBlockInfo {
                        starting_index: usize::try_from(running_starting_index).unwrap(),
                        size: usize::try_from(num_blocks).unwrap(),
                    });
                    for ix in 0..num_blocks {
                        blocks.push(Block { id_number: None });
                    }
                }

                running_starting_index += num_blocks;
            }
        }

        Ok(DiskMap {
            blocks,
            free_blocks_cache: Some(free_blocks),
        })
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

    pub fn defrag_simple(&mut self) {
        let num_elements_pre = self.blocks.len();

        let empty_blocks_indices: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .filter(|&(_, value)| value.id_number.is_none())
            .map(|(index, _)| index)
            .collect();

        let mut file_block_to_defrag: Vec<(usize, u32)> = Vec::new();
        let file_block_end_index = self.blocks.len() - 1;
        for (index, b) in self.blocks.iter().rev().enumerate() {
            if let Some(file_id) = b.id_number {
                file_block_to_defrag.push((file_block_end_index - index, file_id));
            }
        }

        let max_defrag_elements = self
            .blocks
            .iter()
            .filter(|&block| block.id_number.is_some())
            .count();

        let mut to_defrag_iter = file_block_to_defrag.iter();
        for ix in empty_blocks_indices {
            if ix >= max_defrag_elements {
                break;
            }

            let next = to_defrag_iter.next();

            if let Some((index, file_id)) = next {
                self.blocks[ix].id_number = Some(*file_id);
                self.blocks[*index].id_number = None;
            }
        }

        let num_elements_post = self.blocks.len();
        assert_eq!(num_elements_pre, num_elements_post);
    }

    pub fn defrag_to_complete_file(&mut self) {
        todo!()
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(index, value)| {
                if let Some(file_id) = value.id_number {
                    return u64::try_from(file_id).unwrap() * u64::try_from(index).unwrap();
                }
                0
            })
            .sum()
    }
}

pub fn run(config: Config) -> Result<(u64), Box<dyn Error>> {
    let disk_map_raw = fs::read_to_string(config.first_file)?;

    let mut disk_map = DiskMap::make(&disk_map_raw)?;
    disk_map.defrag_simple();
    let checksum = disk_map.checksum();

    Ok((checksum))
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

        disk_map.defrag_simple();

        assert_eq!(
            disk_map.to_string(),
            String::from("0099811188827773336446555566..............")
        );

        assert_eq!(disk_map.checksum(), 1928);
    }
}
