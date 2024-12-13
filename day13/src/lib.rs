use csv::Reader;
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct ClawMachineConfiguration {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    p_x: u32,
    p_y: u32,
}

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

struct ClawMachine {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
    button_a_cost: u32,
    button_b_cost: u32,
}

impl ClawMachine {
    pub fn new(config: &ClawMachineConfiguration) -> Result<ClawMachine, &'static str> {
        Ok(ClawMachine {
            button_a: (config.a_x, config.a_y),
            button_b: (config.b_x, config.b_y),
            prize: (config.p_x, config.p_y),
            button_a_cost: 3u32,
            button_b_cost: 1u32,
        })
    }

    fn get_cost_for_cheapest_combination(&self) -> Option<u32> {
        if let Some(cheapest) = self.compute_cheapest_combination() {
            let cost = cheapest.0 * self.button_a_cost + cheapest.1 * self.button_b_cost;
            return Some(cost);
        }

        None
    }

    fn compute_cheapest_combination(&self) -> Option<(u32, u32)> {
        let combinations = self.compute_all_combinations();

        if let Some(combinations) = combinations {
            return Some(self.get_cheapest_combination(combinations));
        }

        None
    }

    fn compute_all_combinations(&self) -> Option<Vec<(u32, u32)>> {
        let mut combinations = Vec::new();
        for ix in 0u32..=100 {
            for iy in 0u32..=100 {
                let pos = (
                    ix * self.button_a.0 + iy * self.button_b.0,
                    ix * self.button_a.1 + iy * self.button_b.1,
                );

                if pos == self.prize {
                    combinations.push((ix, iy));
                    break;
                }

                if pos > self.prize {
                    break;
                }
            }
        }

        if combinations.is_empty() {
            return None;
        }

        Some(combinations)
    }

    fn get_cheapest_combination(&self, combinations: Vec<(u32, u32)>) -> (u32, u32) {
        let mut cheapest = (0, 0);
        let mut cheapest_cost = 400;
        for c in combinations {
            let cost = c.0 * self.button_a_cost + c.1 * self.button_b_cost;

            if cost < cheapest_cost {
                cheapest = c;
                cheapest_cost = cost;
            }
        }

        cheapest
    }
}

fn deserialize<T, R>(reader: R) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::fmt::Debug + DeserializeOwned,
    R: Read,
{
    let mut rdr = Reader::from_reader(reader);
    let mut structs: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        structs.push(record);
    }

    Ok(structs)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO
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
    fn proper_deserialize_from_slice_to_entry() {
        // Note: must be without spaces
        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
49,27,35,65,4326,4898
82,64,20,67,6818,10409
75,72,95,15,8360,4749
59,26,15,29,7401,3032";

        let structs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();
    }

    #[test]
    fn proper_deserialize_from_file_to_entry() {
        // Note: must be without spaces and by default the base directory should be at the same
        // level of src
        let file = File::open("content/puzzle-input.txt").unwrap();
        let reader = BufReader::new(file);

        let structs: Vec<ClawMachineConfiguration> = deserialize(reader).unwrap();
    }

    #[test]
    fn sample_input_test() {
        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
94,34,22,67,8400,5400
26,66,67,21,12748,12176
17,86,84,37,7870,6450
69,23,27,71,18641,10279";

        let expected = vec![Some((80u32, 40u32)), None, Some((38u32, 86u32)), None];

        let cfgs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();

        let mut total_cost = 0;
        for (index, c) in cfgs.iter().enumerate() {
            let machine = ClawMachine::new(c).unwrap();
            let cheapest = machine.compute_cheapest_combination();

            assert_eq!(cheapest, expected[index]);

            if let Some(cost) = machine.get_cost_for_cheapest_combination() {
                total_cost += cost;
            }
        }

        assert_eq!(total_cost, 480);
    }
}
