use std::fs;
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
        self.do_move(self.position, m);
    }

    fn do_move(&mut self, current: (usize, usize), m: &Move) -> bool {
        assert!(current.0 < self.columns);
        assert!(current.1 < self.rows);

        if self.is_non_movable(current) {
            println!("Not moving from {current:?} because not movable");
            return false;
        }

        let candidate = self.get_pos_from_current_and_move(current, m);
        if let Some(pos) = candidate {
            if self.is_free(pos) || self.do_move(pos, m) {
                println!("Moving to {pos:?}");

                let cur_object = self.positions[current.1][current.0];
                self.positions[pos.1][pos.0] = cur_object;
                self.positions[current.1][current.0] = '.';

                if cur_object == '@' {
                    self.position = pos;
                }

                return true;
            }
        }

        println!("Not moving from {current:?}");
        false
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

    fn is_free(&self, pos: (usize, usize)) -> bool {
        assert!(pos.0 < self.columns);
        assert!(pos.1 < self.rows);

        match self.positions[pos.1][pos.0] {
            '#' | '@' | 'O' => {
                println!("Position {:?} is not free", pos);
                false
            }
            _ => {
                println!("Position {:?} is free", pos);
                true
            }
        }
    }

    fn is_non_movable(&self, pos: (usize, usize)) -> bool {
        assert!(pos.0 < self.columns);
        assert!(pos.1 < self.rows);

        match self.positions[pos.1][pos.0] {
            '#' => {
                println!("Position {:?} is not movable", pos);
                true
            }
            _ => {
                println!("Position {:?} is either free or movable", pos);
                false
            }
        }
    }

    pub fn get_boxes_coordinates_sum(&self) -> u64 {
        let mut total = 0u64;
        for (iy, row) in self.positions.iter().enumerate() {
            for (ix, c) in row.iter().enumerate() {
                if *c == 'O' {
                    total += u64::try_from(ix).unwrap() + 100 * u64::try_from(iy).unwrap();
                }
            }
        }

        total
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

pub fn run(config: Config) -> Result<(u64), Box<dyn Error>> {
    let map_content = fs::read_to_string(config.puzzle_input_map)?;
    let moves_content = fs::read_to_string(config.puzzle_input_moves)?;

    let mut map = WarehouseMap::make(&map_content).unwrap();
    let movements = Moves::make(&moves_content).unwrap();

    movements.moves.iter().for_each(|m| map.update_with_move(m));

    let boxes_coordinates_sum = map.get_boxes_coordinates_sum();
    Ok((boxes_coordinates_sum))
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

        let expected = "\
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";

        let mut actual = String::new();
        for r in map.positions.iter() {
            let row: String = r.iter().collect();
            actual += &format!("{}\n", row);
        }
        assert_eq!(expected, actual.trim());
        assert_eq!(map.get_boxes_coordinates_sum(), 2028);
    }

    #[test]
    fn sample_map_part2_test() {
        let map_data = "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

        let moves_data = "\
<vv<<^^<<^^";

        let mut map = WarehouseMap::make(map_data).unwrap();
        let movements = Moves::make(moves_data).unwrap();

        movements.moves.iter().for_each(|m| map.update_with_move(m));

        let expected = "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";

        let mut actual = String::new();
        for r in map.positions.iter() {
            let row: String = r.iter().collect();
            actual += &format!("{}\n", row);
        }
        assert_eq!(expected, actual.trim());
        assert_eq!(map.get_boxes_coordinates_sum(), 9021);
    }
}
