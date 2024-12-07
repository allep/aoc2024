use csv::Reader;
use serde::de::DeserializeOwned;
use std::fs;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct Rule {
    first_page: i32,
    second_page: i32,
}

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

struct UpdateSet {
    right_order_updates: Vec<Vec<i32>>,
}

impl UpdateSet {
    fn make(raw_content: &str, rules: Vec<Rule>) -> Result<UpdateSet, &'static str> {
        let lines: Vec<&str> = raw_content.trim().split("\n").collect();

        let mut right_order_updates = Vec::new();
        for l in lines {
            let values: Vec<&str> = l.trim().split(",").collect();
            let values: Vec<i32> = values
                .iter()
                .map(|v| v.parse().expect("Cannot convert to i32"))
                .collect();

            if UpdateSet::not_violate_rules(&values, &rules) {
                right_order_updates.push(values);
            }
        }

        Ok(UpdateSet {
            right_order_updates,
        })
    }

    fn not_violate_rules(values: &Vec<i32>, rules: &Vec<Rule>) -> bool {
        true
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
    fn proper_deserialize_from_slice_to_rule() {
        // Note: must be without spaces
        let data = "\
first_page,second_page
47,53
97,13
97,61
";

        let structs: Vec<Rule> = deserialize(data.as_bytes()).unwrap();
    }

    #[test]
    fn proper_deserialize_from_file_to_rule() {
        // Note: must be without spaces and by default the base directory should be at the same
        // level of src
        let file = File::open("content/sample-content.csv").unwrap();
        let reader = BufReader::new(file);

        let structs: Vec<Rule> = deserialize(reader).unwrap();
    }

    #[test]
    fn right_order_update_sample_validation() {
        let rules = "\
first_page,second_page
47,53
97,13
97,61
97,47
75,29
61,13
75,53
29,13
97,29
53,29
61,53
97,53
61,29
47,13
75,47
97,75
47,61
75,61
47,29
75,13
53,13";

        let updates = "\
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let rules: Vec<Rule> = deserialize(rules.as_bytes()).unwrap();
        let updates_set = UpdateSet::make(updates, rules).unwrap();
    }
}
