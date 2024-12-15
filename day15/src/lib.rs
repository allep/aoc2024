use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug)]
pub struct Config {
    puzzle_input_map: String,
    puzzle_input_moves: String,
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

struct Moves {
    moves: Vec<Move>,
}

impl Moves {
    pub fn make(raw_data: &str) -> Result<Moves, &'static str> {
        let raw_moves: Vec<char> = raw_data
            .trim()
            .as_bytes()
            .iter()
            .map(|b| *b as char)
            .filter(|c| *c as char != '\n')
            .collect();

        let mut moves = Vec::new();
        let mut is_ok = true;
        raw_moves.iter().for_each(|c| match *c {
            '^' => moves.push(Move::Up),
            '>' => moves.push(Move::Right),
            'v' => moves.push(Move::Down),
            '<' => moves.push(Move::Left),
            _ => is_ok = false,
        });

        if !is_ok {
            return Err("Some moves were not acceptable");
        }

        Ok(Moves { moves })
    }
}

struct WarehouseMap {
    positions: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    start_position: (usize, usize),
    position: (usize, usize),
}

impl WarehouseMap {
    pub fn make(raw_data: &str) -> Result<WarehouseMap, &'static str> {
        let lines: Vec<String> = raw_data.trim().split("\n").map(|s| s.to_string()).collect();
        let num_rows = lines.len();
        if num_rows == 0 {
            return Err("No row deserialized");
        }

        let mut positions = Vec::new();
        let mut is_ok = true;
        let mut num_columns = 0;
        for l in lines {
            let chars: Vec<char> = l.chars().into_iter().collect();

            let length = chars.len();
            if num_columns == 0 {
                num_columns = length;
            } else {
                if length != num_columns {
                    is_ok = false;
                }
            }

            positions.push(chars);
        }

        if !is_ok {
            return Err("Wrong line length for map.");
        }

        let mut start_position = (0usize, 0usize);
        for (iy, row) in positions.iter().enumerate() {
            for (ix, c) in row.iter().enumerate() {
                if *c == '@' {
                    start_position = (ix, iy);
                    break;
                }
            }
        }

        Ok(WarehouseMap {
            positions,
            rows: num_rows,
            columns: num_columns,
            start_position,
            position: start_position,
        })
    }

    pub fn update_with_move(&mut self, m: &Move) {
        todo!();
    }

    fn can_move(&self, current: (usize, usize), m: &Move) -> bool {
        todo!();
    }

    fn get_pos_from_current_and_move(
        &self,
        current: (usize, usize),
        m: &Move,
    ) -> Option<(usize, usize)> {
        let current = (
            i32::try_from(current.0).unwrap(),
            i32::try_from(current.1).unwrap(),
        );
        let next = match m {
            Move::Up => (current.0, current.1 - 1),
            Move::Right => (current.0 + 1, current.1),
            Move::Down => (current.0, current.1 + 1),
            Move::Left => (current.0 - 1, current.1),
        };

        if self.is_inside_map(next) {
            return Some((
                usize::try_from(next.0).unwrap(),
                usize::try_from(next.1).unwrap(),
            ));
        }

        None
    }

    fn is_inside_map(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && usize::try_from(pos.0).unwrap() < self.columns
            && usize::try_from(pos.1).unwrap() < self.rows
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn start_position(&self) -> (usize, usize) {
        self.start_position
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let puzzle_input_map = args[1].clone();
        let puzzle_input_moves = args[2].clone();

        Ok(Config {
            puzzle_input_map,
            puzzle_input_moves,
        })
    }
}

pub fn run(config: Config) -> Result<(u32), Box<dyn Error>> {
    Ok((0))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn basic_moves_creation_test() {
        let data = "\
<^^>>>vv<v>>v<<";

        let m = Moves::make(data).unwrap();
        assert_eq!(m.moves.len(), 15);
    }

    #[test]
    fn basic_map_creation_test() {
        let data = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let map = WarehouseMap::make(data).unwrap();
        assert_eq!(map.rows(), 8);
        assert_eq!(map.columns(), 8);
        assert_eq!(map.start_position(), (2, 2));
        assert_eq!(map.position(), (2, 2));
    }

    #[test]
    fn sample_map_test() {
        let map_data = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let moves_data = "\
<^^>>>vv<v>>v<<";

        let mut map = WarehouseMap::make(map_data).unwrap();
        let movements = Moves::make(moves_data).unwrap();

        movements.moves.iter().for_each(|m| map.update_with_move(m));
    }
}
