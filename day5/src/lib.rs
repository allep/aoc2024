use csv::Reader;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs;
use std::io::Read;

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
    rules: Vec<Rule>,
    right_order_updates: Vec<Vec<i32>>,
    invalid_order_updates: Vec<Vec<i32>>,
}

impl UpdateSet {
    fn make(raw_content: &str, rules: Vec<Rule>) -> Result<UpdateSet, &'static str> {
        let lines: Vec<&str> = raw_content.trim().split("\n").collect();

        let mut right_order_updates = Vec::new();
        let mut invalid_order_updates = Vec::new();
        for l in lines {
            let values: Vec<&str> = l.trim().split(",").collect();
            let values: Vec<i32> = values
                .iter()
                .map(|v| v.parse().expect("Cannot convert to i32"))
                .collect();

            if UpdateSet::rules_valid(&values, &rules) {
                right_order_updates.push(values);
            } else {
                invalid_order_updates.push(values);
            }
        }

        Ok(UpdateSet {
            rules,
            right_order_updates,
            invalid_order_updates,
        })
    }

    fn rules_valid(values: &Vec<i32>, rules: &Vec<Rule>) -> bool {
        let mut rules_valid = true;
        rules.iter().for_each(|r| {
            let first = values.iter().position(|&x| x == r.first_page);
            let second = values.iter().position(|&x| x == r.second_page);

            if let (Some(first), Some(second)) = (first, second) {
                if first >= second {
                    rules_valid = false;
                }
            }
        });
        rules_valid
    }

    fn right_order_updates(&self) -> usize {
        self.right_order_updates.len()
    }

    fn right_ordered_middle_page_numbers_sum(&self) -> u32 {
        let mut sum: u32 = 0;
        self.right_order_updates.iter().for_each(|update| {
            assert!(update.len() % 2 == 1);

            let index = update.len() / 2;
            sum += u32::try_from(update[index]).unwrap();
        });

        sum
    }

    fn order_wrong_updates_by_rules(&mut self) {
        todo!();
    }

    fn wrong_ordered_middle_page_number_sum(&self) -> u32 {
        let mut sum: u32 = 0;
        self.invalid_order_updates.iter().for_each(|update| {
            assert!(update.len() % 2 == 1);

            let index = update.len() / 2;
            sum += u32::try_from(update[index]).unwrap();
        });

        sum
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

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let rules = fs::read_to_string(config.first_file)?;
    let updates = fs::read_to_string(config.second_file)?;

    let rules: Vec<Rule> = deserialize(rules.as_bytes()).unwrap();
    let updates = UpdateSet::make(&updates, rules).unwrap();
    Ok(updates.right_ordered_middle_page_numbers_sum())
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

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
        assert_eq!(structs.len(), 3);
    }

    #[test]
    fn proper_deserialize_from_file_to_rule() {
        // Note: must be without spaces and by default the base directory should be at the same
        // level of src
        let file = File::open("content/sample-content.csv").unwrap();
        let reader = BufReader::new(file);

        let structs: Vec<Rule> = deserialize(reader).unwrap();
        assert_eq!(structs.len(), 21);
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

        assert_eq!(updates_set.right_order_updates(), 3);
        assert_eq!(updates_set.right_ordered_middle_page_numbers_sum(), 143);
    }

    #[test]
    fn violation_validation() {
        let rules = vec![(47, 53), (97, 13), (97, 61)];
        let update = vec![75, 47, 61, 53, 29];

        let mut rules_valid = true;
        rules.iter().for_each(|&r| {
            let first = update.iter().position(|&x| x == r.0);
            let second = update.iter().position(|&x| x == r.1);

            if let (Some(first), Some(second)) = (first, second) {
                if first >= second {
                    rules_valid = false;
                }
            }
        });

        assert!(rules_valid);
    }

    #[test]
    fn invalid_order_update_sample_validation() {
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
        let mut updates_set = UpdateSet::make(updates, rules).unwrap();

        updates_set.order_wrong_updates_by_rules();
        assert_eq!(updates_set.wrong_ordered_middle_page_number_sum(), 123);
    }
}
