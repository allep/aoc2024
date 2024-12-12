use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
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

#[derive(Debug)]
pub struct AntennasMap {
    antinodes_positions: HashMap<char, Vec<(usize, usize)>>,
}

impl AntennasMap {
    pub fn count_unique_antinodes(&self) -> usize {
        let mut unique_antinodes = HashSet::new();
        self.antinodes_positions.iter().for_each(|element| {
            element.1.iter().for_each(|pos| {
                unique_antinodes.insert(*pos);
            })
        });

        unique_antinodes.len()
    }
}

struct AntennasMapFactory {}

impl AntennasMapFactory {
    pub fn make(raw_content: &str) -> Result<AntennasMap, &'static str> {
        let lines: Vec<String> = raw_content
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        if lines.len() == 0 {
            return Err("No lines read from raw content.");
        }

        if lines[0].len() == 0 {
            return Err("Read empty line.");
        }

        let y_max = lines.len();
        let x_max = lines[0].len();

        let antennas_positions = Self::compute_antenna_positions(&lines);
        let antinodes_positions =
            Self::compute_antinode_positions(antennas_positions, x_max, y_max);

        Ok(AntennasMap {
            antinodes_positions,
        })
    }

    fn compute_antenna_positions(lines: &Vec<String>) -> HashMap<char, Vec<(usize, usize)>> {
        let mut positions = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.char_indices() {
                if c.is_ascii_alphanumeric() {
                    positions
                        .entry(c)
                        .and_modify(|list: &mut Vec<(usize, usize)>| list.push((x, y)))
                        .or_insert(vec![(x, y)]);
                }
            }
        }

        positions
    }

    fn compute_antinode_positions(
        antennas_positions: HashMap<char, Vec<(usize, usize)>>,
        x_max: usize,
        y_max: usize,
    ) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antinodes_map = HashMap::new();
        for (frequency, positions) in antennas_positions {
            let mut antinodes_for_frequency = Vec::new();
            for (index, position) in positions.iter().enumerate() {
                let other_positions = &positions[index + 1..];

                for other_position in other_positions {
                    let distance = Self::compute_distance(*position, *other_position);

                    let mut antinodes = Self::compute_antinodes_for_antenna_pair(
                        *position,
                        *other_position,
                        distance,
                        x_max,
                        y_max,
                    );

                    antinodes_for_frequency.append(&mut antinodes);
                }
            }

            antinodes_map.insert(frequency, antinodes_for_frequency);
        }

        antinodes_map
    }

    fn compute_distance(
        first_antenna: (usize, usize),
        second_antenna: (usize, usize),
    ) -> (i32, i32) {
        (
            i32::try_from(second_antenna.0).unwrap() - i32::try_from(first_antenna.0).unwrap(),
            i32::try_from(second_antenna.1).unwrap() - i32::try_from(first_antenna.1).unwrap(),
        )
    }

    fn is_valid_antinode(position: (i32, i32), x_max: usize, y_max: usize) -> bool {
        position.0 >= 0
            && usize::try_from(position.0).unwrap() < x_max
            && position.1 >= 0
            && usize::try_from(position.1).unwrap() < y_max
    }

    fn compute_antinodes_for_antenna_pair(
        first_antenna: (usize, usize),
        second_antenna: (usize, usize),
        distance: (i32, i32),
        x_max: usize,
        y_max: usize,
    ) -> Vec<(usize, usize)> {
        let mut antinodes = Vec::new();

        let mut antinode = (
            i32::try_from(first_antenna.0).unwrap(),
            i32::try_from(first_antenna.1).unwrap(),
        );

        loop {
            antinode = (antinode.0 - distance.0, antinode.1 - distance.1);
            if Self::is_valid_antinode(antinode, x_max, y_max) {
                antinodes.push(antinode);
            } else {
                break;
            }
        }

        let mut antinode = (
            i32::try_from(second_antenna.0).unwrap(),
            i32::try_from(second_antenna.1).unwrap(),
        );

        loop {
            antinode = (antinode.0 + distance.0, antinode.1 + distance.1);
            if Self::is_valid_antinode(antinode, x_max, y_max) {
                antinodes.push(antinode);
            } else {
                break;
            }
        }

        antinodes
            .iter()
            .map(|position| {
                (
                    usize::try_from(position.0).unwrap(),
                    usize::try_from(position.1).unwrap(),
                )
            })
            .collect()
    }

    fn filter_valid_antinodes(
        antinodes: Vec<(i32, i32)>,
        x_max: usize,
        y_max: usize,
    ) -> Vec<(usize, usize)> {
        let mut valid = Vec::new();
        for a in antinodes {
            if Self::is_valid_antinode(a, x_max, y_max) {
                valid.push((usize::try_from(a.0).unwrap(), usize::try_from(a.1).unwrap()));
            }
        }

        valid
    }
}

impl AntennasMap {}

pub fn run(config: Config) -> Result<(usize), Box<dyn Error>> {
    let raw_content = fs::read_to_string(config.puzzle_input)?;
    let antennas_map = AntennasMapFactory::make(&raw_content).unwrap();
    let unique_antinodes = antennas_map.count_unique_antinodes();
    Ok((unique_antinodes))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn compute_antenna_positions_test() {
        let data = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let antennas_map = AntennasMapFactory::make(data).unwrap();
        assert_eq!(antennas_map.count_unique_antinodes(), 34);
    }
}
