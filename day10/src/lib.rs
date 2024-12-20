use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
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

struct TrailPosition {
    height: u32,
    position: (usize, usize),
    next: Option<Vec<Box<TrailPosition>>>,
}

struct TopographicMap {
    positions: Vec<Vec<char>>,
    x_max: usize,
    y_max: usize,
    trailheads: Vec<(usize, usize)>,
    scores: HashMap<(usize, usize), HashSet<(usize, usize)>>,
    ratings: HashMap<(usize, usize), HashSet<Vec<(usize, usize)>>>,
}

impl TopographicMap {
    pub fn make(raw_data: &str) -> Result<TopographicMap, &'static str> {
        let positions: Vec<Vec<char>> = raw_data
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();

        if positions.len() == 0 {
            return Err("No lines read from raw content.");
        }

        if positions[0].len() == 0 {
            return Err("Read empty line.");
        }

        let y_max = positions.len();
        let x_max = positions[0].len();

        Ok(TopographicMap {
            positions,
            x_max,
            y_max,
            trailheads: Vec::new(),
            scores: HashMap::new(),
            ratings: HashMap::new(),
        })
    }

    fn is_position_valid(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && usize::try_from(position.0).unwrap() < self.x_max
            && position.1 >= 0
            && usize::try_from(position.1).unwrap() < self.y_max
    }

    fn is_expected_height_at_position(
        &self,
        position: &(usize, usize),
        expected_height: u32,
    ) -> bool {
        assert!(position.0 < self.x_max);
        assert!(position.1 < self.y_max);

        let actual_height = self.positions[position.1][position.0].to_digit(10).unwrap();
        actual_height == expected_height
    }

    fn get_next_height(&self, current_height: u32) -> u32 {
        current_height + 1
    }

    fn compute_trailheads(&mut self) {
        for (y, line) in self.positions.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '0' {
                    self.trailheads.push((x, y));
                }
            }
        }

        let trailheads = self.trailheads.clone();
        for th in trailheads.iter() {
            let hiking_trail = vec![*th];
            let ht = self.compute_hiking_trail_recursive(*th, hiking_trail, *th, None, 0);
        }
    }

    fn sum_scores(&self) -> u32 {
        let mut total = 0u32;
        for s in &self.scores {
            total += u32::try_from(s.1.len()).unwrap();
        }

        total
    }

    fn sum_ratings(&self) -> u32 {
        let mut total = 0u32;
        for s in &self.ratings {
            total += u32::try_from(s.1.len()).unwrap();
        }

        total
    }

    fn compute_hiking_trail_recursive(
        &mut self,
        trailhead: (usize, usize),
        hiking_trail: Vec<(usize, usize)>,
        current: (usize, usize),
        from: Option<(usize, usize)>,
        current_height: u32,
    ) -> TrailPosition {
        if current_height == 9 {
            // found top
            self.scores
                .entry(trailhead)
                .and_modify(|set| {
                    set.insert(current);
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(current);
                    set
                });

            self.ratings
                .entry(trailhead)
                .and_modify(|set| {
                    set.insert(hiking_trail.clone());
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(hiking_trail.clone());
                    set
                });
        }

        let possible_positions = self.get_possible_valid_positions(&current, from, current_height);

        if let None = possible_positions {
            return TrailPosition {
                height: current_height,
                position: current,
                next: None,
            };
        }

        let mut next = None;
        if let Some(positions) = possible_positions {
            let mut trails = Vec::new();
            for p in positions.iter() {
                let mut hiking_trail = hiking_trail.clone();
                let new_pos = vec![*p];
                hiking_trail.extend(&new_pos);
                let trail = self.compute_hiking_trail_recursive(
                    trailhead,
                    hiking_trail,
                    *p,
                    Some(current),
                    self.get_next_height(current_height),
                );

                trails.push(Box::new(trail));
            }

            if !trails.is_empty() {
                next = Some(trails);
            }
        }

        TrailPosition {
            height: current_height,
            position: current,
            next,
        }
    }

    fn get_possible_valid_positions(
        &self,
        current_pos: &(usize, usize),
        from: Option<(usize, usize)>,
        current_height: u32,
    ) -> Option<Vec<(usize, usize)>> {
        let candidates: Vec<(i32, i32)> = vec![
            (
                i32::try_from(current_pos.0).unwrap() - 1,
                i32::try_from(current_pos.1).unwrap(),
            ),
            (
                i32::try_from(current_pos.0).unwrap(),
                i32::try_from(current_pos.1).unwrap() - 1,
            ),
            (
                i32::try_from(current_pos.0).unwrap() + 1,
                i32::try_from(current_pos.1).unwrap(),
            ),
            (
                i32::try_from(current_pos.0).unwrap(),
                i32::try_from(current_pos.1 + 1).unwrap(),
            ),
        ];

        let candidates: Vec<(usize, usize)> = candidates
            .iter()
            .filter(|position| self.is_position_valid(**position))
            .map(|position| {
                (
                    usize::try_from(position.0).unwrap(),
                    usize::try_from(position.1).unwrap(),
                )
            })
            .filter(|position| {
                let next_height = self.get_next_height(current_height);
                self.is_expected_height_at_position(position, next_height)
            })
            .filter(|position| {
                if let Some(from_position) = from {
                    return from_position != *position;
                }
                return true;
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        Some(candidates)
    }

    fn trailheads_num(&self) -> usize {
        self.trailheads.len()
    }
}

pub fn run(config: Config) -> Result<(u32, u32), Box<dyn Error>> {
    let raw_content = fs::read_to_string(config.puzzle_input)?;

    let mut topographic_map = TopographicMap::make(&raw_content).unwrap();
    topographic_map.compute_trailheads();

    let tot_score = topographic_map.sum_scores();
    let tot_ratings = topographic_map.sum_ratings();

    Ok((tot_score, tot_ratings))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn simple_count_trailhead_test() {
        let data = "
9990999
9991999
9992999
6543456
7111117
8111118
9111119";

        let mut topographic_map = TopographicMap::make(data).unwrap();
        topographic_map.compute_trailheads();

        let num_trailheads = topographic_map.trailheads_num();
        assert_eq!(num_trailheads, 1);

        let tot_score = topographic_map.sum_scores();
        assert_eq!(tot_score, 2);
    }

    #[test]
    fn count_trailhead_test() {
        let data = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let mut topographic_map = TopographicMap::make(data).unwrap();
        topographic_map.compute_trailheads();

        let num_trailheads = topographic_map.trailheads_num();
        assert_eq!(num_trailheads, 9);

        let tot_score = topographic_map.sum_scores();
        assert_eq!(tot_score, 36);
    }
}
