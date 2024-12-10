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

struct BlockInfo {
    starting_index: usize,
    size: usize,
}

struct DiskMap {
    blocks: Vec<Block>,
    free_blocks_cache: Option<Vec<BlockInfo>>,
    file_blocks_cache: Option<Vec<BlockInfo>>,
}

impl DiskMap {
    pub fn make(raw_data: &str) -> Result<DiskMap, &'static str> {
        let mut blocks = Vec::new();
        let mut free_blocks = Vec::new();
        let mut file_blocks = Vec::new();
        let mut id_number = 0;
        let mut running_starting_index: u32 = 0;
        for (index, c) in raw_data.char_indices() {
            if let Some(num_blocks) = c.to_digit(10) {
                let block_info = BlockInfo {
                    starting_index: usize::try_from(running_starting_index).unwrap(),
                    size: usize::try_from(num_blocks).unwrap(),
                };
                if index % 2 == 0 {
                    file_blocks.push(block_info);
                    for ix in 0..num_blocks {
                        blocks.push(Block {
                            id_number: Some(id_number),
                        });
                    }

                    id_number += 1;
                } else {
                    if num_blocks > 0 {
                        free_blocks.push(block_info);
                        for ix in 0..num_blocks {
                            blocks.push(Block { id_number: None });
                        }
                    }
                }

                running_starting_index += num_blocks;
            }
        }

        Ok(DiskMap {
            blocks,
            free_blocks_cache: Some(free_blocks),
            file_blocks_cache: Some(file_blocks),
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

    pub fn free_blocks_info_to_string(&self) -> String {
        let mut representation = String::new();

        if let Some(free_blocks) = &self.free_blocks_cache {
            for f in free_blocks {
                representation += &format!("[start = {}, num = {}]\n", f.starting_index, f.size);
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
        if let (Some(free_blocks), Some(file_blocks)) =
            (&mut self.free_blocks_cache, &mut self.file_blocks_cache)
        {
            let mut files_iter = file_blocks.iter().rev().enumerate();

            while let Some((file_info_index, block_info)) = files_iter.next() {
                let mut free_iter = free_blocks.iter().enumerate();
                let mut free_block_to_cleanup = None;
                while let Some((info_index, free_block_info)) = free_iter.next() {
                    if block_info.size <= free_block_info.size {
                        println!(
                            "Found free block at index = {} for file with size {}",
                            free_block_info.starting_index, block_info.size
                        );

                        free_block_to_cleanup = Some(info_index);

                        // actual move
                        let to_start_index = free_block_info.starting_index;
                        let from_start_index = block_info.starting_index;
                        let from_size = block_info.size;

                        for ix in 0..from_size {
                            self.blocks[to_start_index + ix].id_number =
                                self.blocks[from_start_index + ix].id_number;

                            self.blocks[from_start_index + ix].id_number = None;
                        }

                        break;
                    } else {
                        println!(
                            "Found free block at index = {} with no enough size",
                            free_block_info.starting_index
                        );
                    }
                }

                if let Some(free_index) = free_block_to_cleanup {
                    // update cache first
                    free_blocks.remove(free_index);
                    println!("Moving file ...");
                }
            }
        }

        // get next candidate to move
        // loop over possible free blocks
        // if Some -> move
        //  - remove the file block
        //  - update free blocks
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

    #[ignore]
    #[test]
    fn sample_input_validation() {
        let data = "\
2333133121414131402";

        let mut disk_map = DiskMap::make(data).unwrap();
        assert_eq!(
            disk_map.to_string(),
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );

        println!(
            "Free blocks info:\n{}",
            disk_map.free_blocks_info_to_string()
        );

        disk_map.defrag_simple();

        assert_eq!(
            disk_map.to_string(),
            String::from("0099811188827773336446555566..............")
        );

        assert_eq!(disk_map.checksum(), 1928);
    }

    #[test]
    fn part2_logic_validation() {
        let data = "\
2333133121414131402";

        let mut disk_map = DiskMap::make(data).unwrap();
        disk_map.defrag_to_complete_file();

        assert_eq!(
            disk_map.to_string(),
            String::from("00992111777.44.333....5555.6666.....8888..")
        );

        assert_eq!(disk_map.checksum(), 2858);
    }
}
