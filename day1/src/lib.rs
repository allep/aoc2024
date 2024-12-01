use csv::Reader;
use serde::de::DeserializeOwned;
use std::io::{self, BufReader, Read};
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize)]
struct Entry {
    left_list: i32,
    right_list: i32,
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

fn get_total_distance_from_raw_data(raw_list: &Vec<Entry>) -> i32 {
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

    let length = raw_list.len();

    assert_eq!(first.len(), length);
    assert_eq!(second.len(), length);

    let mut total_distance = 0;
    for ix in 0..length {
        let left = first[ix];
        let right = second[ix];

        let diff = right - left;

        // Need to account for absolute distances
        total_distance += diff.abs();
    }

    total_distance
}

fn get_total_similarity_score_from_raw_data(raw_list: &Vec<Entry>) -> i32 {
    let mut first = Vec::new();
    let mut second = Vec::new();

    let _ = raw_list
        .iter()
        .map(|x| {
            first.push(x.left_list);
            second.push(x.right_list);
        })
        .collect::<Vec<_>>();

    let mut score = 0;
    for x in first {
        let occurrences = second.iter().filter(|&n| *n == x).count();
        score += x * occurrences as i32;
    }

    score
}

pub fn run(config: Config) -> Result<(i32, i32), Box<dyn Error>> {
    let file = File::open(config.puzzle_input)?;
    let reader = BufReader::new(file);

    let structs: Vec<Entry> = deserialize(reader)?;
    let total_distance = get_total_distance_from_raw_data(&structs);
    let similarity_score = get_total_similarity_score_from_raw_data(&structs);

    Ok((total_distance, similarity_score))
}

// Note on printing during tests:
// - Run test sequentially in case of need with: cargo test -- --test-threads 1
// - Do not capture test output for debug with: cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn total_distance_verify() {
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
        let total_distance = get_total_distance_from_raw_data(&structs);
        assert_eq!(total_distance, 11);
    }

    #[test]
    fn total_similarity_score_verify() {
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
        let total_similarity_score = get_total_similarity_score_from_raw_data(&structs);
        assert_eq!(total_similarity_score, 31);
    }
}
