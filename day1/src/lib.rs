use csv::Reader;
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct Entry {
    left_list: i32,
    right_list: i32,
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

fn compute_distance(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    todo!();
}

fn get_total_distance_from_raw_data(raw_list: Vec<Entry>) -> i32 {
    println!("Raw list is: {raw_list:?}");
    let mut first = Vec::new();
    let mut second = Vec::new();

    let _ = raw_list
        .iter()
        .map(|x| {
            first.push(x.left_list);
            second.push(x.right_list);
        })
        .collect::<Vec<_>>();

    first.sort();
    second.sort();

    println!("Sorted first: {first:?}");
    println!("Sorted second: {second:?}");

    let length = raw_list.len();

    assert_eq!(first.len(), length);
    assert_eq!(second.len(), length);

    let mut total_distance = 0;
    for ix in 0..length {
        let left = first[ix];
        let right = second[ix];

        assert!(left <= right);

        let diff = right - left;
        total_distance += diff;
    }

    total_distance
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

    #[ignore]
    #[test]
    fn proper_deserialize_from_file_to_entry() {
        // Note: must be without spaces and by default the base directory should be at the same
        // level of src
        let file = File::open("content/sample-content.csv").unwrap();
        let reader = BufReader::new(file);

        let structs: Vec<Entry> = deserialize(reader).unwrap();
    }

    #[test]
    fn make_paried_list_verify() {
        let data = "\
left_list,right_list
3,4
4,3
2,5
1,3
3,9
3,3
";
        let structs: Vec<Entry> = deserialize(data.as_bytes()).unwrap();
        let total_distance = get_total_distance_from_raw_data(structs);
        assert_eq!(total_distance, 11);
    }
}
