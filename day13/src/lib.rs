use csv::Reader;
use io::BufReader;
use serde::de::DeserializeOwned;
use std::cmp;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct ClawMachineConfiguration {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    p_x: u64,
    p_y: u64,
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

#[derive(Debug)]
enum StepsByDimension {
    X(u64),
    Y(u64),
}

impl StepsByDimension {
    fn get_steps(&self) -> u64 {
        match self {
            StepsByDimension::X(steps) | StepsByDimension::Y(steps) => *steps,
        }
    }
}

enum Movement {
    A((u64, u64)),
    B((u64, u64)),
}

impl Movement {
    fn get_steps_upper_bound(&self, distance: (u64, u64)) -> StepsByDimension {
        match self {
            Movement::A(movement) | Movement::B(movement) => {
                let x = distance.0 / movement.0;
                let y = distance.1 / movement.1;

                if x < y {
                    println!("- Using dimension x");
                    StepsByDimension::X(x)
                } else {
                    println!("- Using dimension y");
                    StepsByDimension::Y(y)
                }
            }
        }
    }

    fn get_steps_upper_bound_for_x(&self, distance: (u64, u64)) -> StepsByDimension {
        println!("- Using dimension x now");
        match self {
            Movement::A(movement) | Movement::B(movement) => {
                let x = distance.0 / movement.0;
                StepsByDimension::X(x)
            }
        }
    }

    fn get_steps_upper_bound_for_y(&self, distance: (u64, u64)) -> StepsByDimension {
        println!("- Using dimension y now");
        match self {
            Movement::A(movement) | Movement::B(movement) => {
                let y = distance.1 / movement.1;
                StepsByDimension::Y(y)
            }
        }
    }

    fn get_distance_for_step(&self, steps: u64) -> (u64, u64) {
        match self {
            Movement::A(movement) | Movement::B(movement) => {
                (steps * movement.0, steps * movement.1)
            }
        }
    }

    fn get_steps(&self) -> (u64, u64) {
        match self {
            Movement::A(movement) | Movement::B(movement) => *movement,
        }
    }
}

struct ClawMachine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
    button_a_cost: u64,
    button_b_cost: u64,
    epsilon: i64,
    learning_rate: i64,
}

impl ClawMachine {
    pub fn new(
        config: &ClawMachineConfiguration,
        epsilon: i64,
        learning_rate: i64,
    ) -> Result<ClawMachine, &'static str> {
        Ok(ClawMachine {
            button_a: (config.a_x, config.a_y),
            button_b: (config.b_x, config.b_y),
            prize: (config.p_x, config.p_y),
            button_a_cost: 3u64,
            button_b_cost: 1u64,
            epsilon,
            learning_rate,
        })
    }

    fn get_cost_for_cheapest_combination_heuristic(&self) -> Option<u64> {
        if let Some(cheapest) = self.compute_cheapest_combination_heuristic() {
            let cost = cheapest.0 * self.button_a_cost + cheapest.1 * self.button_b_cost;
            return Some(cost);
        }

        None
    }

    fn get_cost_for_cheapest_combination(&self) -> Option<u64> {
        if let Some(cheapest) = self.compute_cheapest_combination() {
            let cost = cheapest.0 * self.button_a_cost + cheapest.1 * self.button_b_cost;
            return Some(cost);
        }

        None
    }

    fn compute_cheapest_combination_heuristic(&self) -> Option<(u64, u64)> {
        let combinations = self.compute_heuristic_combinations();

        if let Some(combinations) = combinations {
            return Some(self.get_cheapest_combination(combinations));
        }

        None
    }

    fn compute_cheapest_combination(&self) -> Option<(u64, u64)> {
        let combinations = self.compute_all_combinations();

        if let Some(combinations) = combinations {
            return Some(self.get_cheapest_combination(combinations));
        }

        None
    }

    fn compute_steepest_descend_combinations(&self) -> Option<Vec<(u64, u64)>> {
        let mut a = 10000000012748i64;
        let mut b = 10000000012176i64;

        for ix in 0..1000000000 {
            let remainder = (
                i64::try_from(self.prize.0).unwrap()
                    - a * i64::try_from(self.button_a.0).unwrap()
                    - b * i64::try_from(self.button_b.0).unwrap(),
                i64::try_from(self.prize.1).unwrap()
                    - a * i64::try_from(self.button_a.1).unwrap()
                    - b * i64::try_from(self.button_b.1).unwrap(),
            );
            if ix % 1000000 == 0 {
                println!(
                    "Done another 1000000 iterations. Current pos: ({}, {}), remainder: ({}, {}), cost: {}",
                    a,
                    b,
                    remainder.0,
                    remainder.1,
                    self.cost_function(a, b)
                );
            }

            let grad = self.gradient(a, b);

            let a_new = i64::try_from(a).unwrap() - self.learning_rate * grad.0;
            let b_new = i64::try_from(b).unwrap() - self.learning_rate * grad.1;

            a = a_new;
            b = b_new;
        }

        let a = u64::try_from(a).unwrap();
        let b = u64::try_from(b).unwrap();

        None
    }

    fn gradient(&self, steps_a: i64, steps_b: i64) -> (i64, i64) {
        let fx_plus = self.cost_function(steps_a + self.epsilon, steps_b);
        let fx_minus = self.cost_function(steps_a - self.epsilon, steps_b);
        let fy_plus = self.cost_function(steps_a, steps_b + self.epsilon);
        let fy_minus = self.cost_function(steps_a, steps_b - self.epsilon);

        let dx = (i64::try_from(fx_plus).unwrap() - i64::try_from(fx_minus).unwrap())
            / (2 * i64::try_from(self.epsilon).unwrap());
        let dy = (i64::try_from(fy_plus).unwrap() - i64::try_from(fy_minus).unwrap())
            / (2 * i64::try_from(self.epsilon).unwrap());

        (dx, dy)
    }

    fn cost_function(&self, steps_a: i64, steps_b: i64) -> u64 {
        let a = steps_a;
        let b = steps_b;
        let a_x = i64::try_from(self.button_a.0).unwrap();
        let a_y = i64::try_from(self.button_a.1).unwrap();
        let b_x = i64::try_from(self.button_b.0).unwrap();
        let b_y = i64::try_from(self.button_b.1).unwrap();
        let p_x = i64::try_from(self.prize.0).unwrap();
        let p_y = i64::try_from(self.prize.1).unwrap();

        let mut cost_x = (p_x - a_x * a - b_x * b);
        let mut cost_y = (p_y - a_y * a - b_y * b);

        if cost_x < 0 {
            cost_x = -cost_x;
        }

        if cost_y < 0 {
            cost_y = -cost_y;
        }

        let cost = cost_x + cost_y;

        u64::try_from(cost).unwrap()
    }

    fn compute_heuristic_combinations(&self) -> Option<Vec<(u64, u64)>> {
        let mut combinations = Vec::new();

        let eff_a = Self::get_efficiency(self.button_a, self.button_a_cost);
        let eff_b = Self::get_efficiency(self.button_b, self.button_b_cost);

        let movement: Movement;
        let other_movement: Movement;
        if eff_a > eff_b {
            println!("Using main movement = A");
            movement = Movement::A(self.button_a);
            other_movement = Movement::B(self.button_b);
        } else {
            println!("Using main movement = B");
            movement = Movement::B(self.button_b);
            other_movement = Movement::A(self.button_a);
        }

        let mut main_movement_steps_num = 0;
        let mut other_movement_steps_num = 0;
        let mut other_movement_diff = (0u64, 0u64);

        for ix in 1..10 {
            // early break in case of odd values
            if other_movement_diff.0 > self.prize.0 || other_movement_diff.1 > self.prize.1 {
                println!("Can't reach the prize, breaking");
                break;
            }

            let updated_distance = (
                self.prize.0 - other_movement_diff.0,
                self.prize.1 - other_movement_diff.1,
            );
            let steps_by_dimension = movement.get_steps_upper_bound_for_x(updated_distance);
            main_movement_steps_num = steps_by_dimension.get_steps();

            let current_combination = (main_movement_steps_num, other_movement_steps_num);
            println!(
                "Current combination of steps: ({}, {})",
                current_combination.0, current_combination.1
            );

            let remainder = Self::get_remainder(
                self.prize,
                &movement,
                main_movement_steps_num,
                &other_movement,
                other_movement_steps_num,
            );

            if let (0u64, 0u64) = remainder {
                println!("Found prize, returning");

                combinations.push(current_combination);
                break;
            }

            println!("- Current remainder: {:?}", remainder);

            // prize not found, move forward and update movements

            let other_dimension_steps = match steps_by_dimension {
                StepsByDimension::X(_) => {
                    // need to use Y dimension now with the other movement
                    other_movement.get_steps_upper_bound_for_y(remainder)
                }
                StepsByDimension::Y(_) => {
                    // need to use X dimension now with the other movement
                    other_movement.get_steps_upper_bound_for_x(remainder)
                }
            };

            let other_cur_steps = other_dimension_steps.get_steps();

            if other_cur_steps == other_movement_steps_num {
                println!("No chance to reach prize, stuck in same position, breaking");
                break;
            }

            other_movement_steps_num = other_cur_steps;
            other_movement_diff = other_movement.get_distance_for_step(other_movement_steps_num);
        }

        if combinations.is_empty() {
            println!("Didn't find any result ...");
            return None;
        }

        Some(combinations)
    }

    fn get_efficiency(movement: (u64, u64), cost: u64) -> f64 {
        let squared_mag = (movement.0.pow(2) + movement.1.pow(2)) as f64;
        squared_mag / cost as f64
    }

    fn get_remainder(
        prize: (u64, u64),
        movement: &Movement,
        steps: u64,
        other_movement: &Movement,
        other_steps: u64,
    ) -> (u64, u64) {
        (
            prize.0 - steps * movement.get_steps().0 - other_steps * other_movement.get_steps().0,
            prize.1 - steps * movement.get_steps().1 - other_steps * other_movement.get_steps().1,
        )
    }

    fn is_prize_reached(
        &self,
        main_m: &Movement,
        other_m: &Movement,
        main_steps: u64,
        other_steps: u64,
    ) -> bool {
        match (main_m, other_m) {
            (Movement::A((x_a, y_a)), Movement::B((x_b, y_b))) => {
                (
                    x_a * main_steps + x_b * other_steps,
                    y_a * main_steps + y_b * other_steps,
                ) == self.prize
            }
            (Movement::B((x_b, y_b)), Movement::A((x_a, y_a))) => {
                (
                    x_b * main_steps + x_a * other_steps,
                    y_b * main_steps + y_a * other_steps,
                ) == self.prize
            }
            (_, _) => false,
        }
    }

    fn is_over_prize(
        &self,
        main_m: &Movement,
        other_m: &Movement,
        main_steps: u64,
        other_steps: u64,
    ) -> bool {
        match (main_m, other_m) {
            (Movement::A((x_a, y_a)), Movement::B((x_b, y_b))) => {
                (
                    x_a * main_steps + x_b * other_steps,
                    y_a * main_steps + y_b * other_steps,
                ) > self.prize
            }
            (Movement::B((x_b, y_b)), Movement::A((x_a, y_a))) => {
                (
                    x_b * main_steps + x_a * other_steps,
                    y_b * main_steps + y_a * other_steps,
                ) > self.prize
            }
            (_, _) => false,
        }
    }

    fn compute_all_combinations(&self) -> Option<Vec<(u64, u64)>> {
        let mut combinations = Vec::new();
        for ix in 0u64..=100 {
            for iy in 0u64..=100 {
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

    fn get_cheapest_combination(&self, combinations: Vec<(u64, u64)>) -> (u64, u64) {
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

pub fn run(config: Config) -> Result<(u64), Box<dyn Error>> {
    let file = File::open(config.puzzle_input)?;
    let reader = BufReader::new(file);

    let cfgs: Vec<ClawMachineConfiguration> = deserialize(reader).unwrap();

    let mut total_cost = 0;
    for (index, c) in cfgs.iter().enumerate() {
        let machine = ClawMachine::new(c, 1, 1i64).unwrap();
        if let Some(cost) = machine.get_cost_for_cheapest_combination() {
            total_cost += cost;
        }
    }

    Ok((total_cost))
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

    #[ignore]
    #[test]
    fn sample_input_test() {
        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
94,34,22,67,8400,5400
26,66,67,21,12748,12176
17,86,84,37,7870,6450
69,23,27,71,18641,10279";

        let expected = vec![Some((80u64, 40u64)), None, Some((38u64, 86u64)), None];

        let cfgs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();

        let mut total_cost = 0;
        for (index, c) in cfgs.iter().enumerate() {
            let machine = ClawMachine::new(c, 1, 1i64).unwrap();
            let cheapest = machine.compute_cheapest_combination();

            assert_eq!(cheapest, expected[index]);

            if let Some(cost) = machine.get_cost_for_cheapest_combination() {
                total_cost += cost;
            }
        }

        assert_eq!(total_cost, 480);
    }

    #[test]
    fn sample_input_part2_test() {
        //let data = "\
        //a_x,a_y,b_x,b_y,p_x,p_y
        //94,34,22,67,10000000008400,10000000005400
        //26,66,67,21,10000000012748,10000000012176
        //17,86,84,37,10000000007870,10000000006450
        //69,23,27,71,10000000018641,10000000010279";

        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
26,66,67,21,10000000012748,10000000012176";

        let cfgs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();

        let mut total_cost = 0;
        for (index, c) in cfgs.iter().enumerate() {
            println!("Running machine {index}...");
            let machine = ClawMachine::new(c, 1, 1i64).unwrap();
            let cheapest = machine.compute_cheapest_combination();

            if let Some(cost) = machine.get_cost_for_cheapest_combination_heuristic() {
                println!("Found cost = {cost}");
                total_cost += cost;
            }
        }

        assert_eq!(total_cost, 480);
    }

    #[ignore]
    #[test]
    fn sample_input_steepest_descend_part2_test() {
        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
26,66,67,21,10000000012748,10000000012176";

        let cfgs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();

        let mut total_cost = 0;
        for (index, c) in cfgs.iter().enumerate() {
            println!("Running machine {index}...");
            let machine = ClawMachine::new(c, 100, 1i64).unwrap();
            let cheapest = machine.compute_cheapest_combination();

            if let Some(cost) = machine.compute_steepest_descend_combinations() {
                println!("Found cost possible costs");
            }
        }

        assert_eq!(total_cost, 480);
    }

    #[test]
    fn cost_function_test() {
        let data = "\
a_x,a_y,b_x,b_y,p_x,p_y
26,66,67,21,10000000012748,10000000012176";

        let cfgs: Vec<ClawMachineConfiguration> = deserialize(data.as_bytes()).unwrap();
        for (index, c) in cfgs.iter().enumerate() {
            println!("Running machine {index}...");
            let machine = ClawMachine::new(c, 100, 100).unwrap();

            let steps_a = 103200496270i64;
            let steps_b = 118675644717i64;

            machine.cost_function(steps_a, steps_b);
            machine.cost_function(0, 0);
        }
    }
}
