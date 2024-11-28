use csv::Reader;
use std::{error::Error, fs::File, io, process};

#[derive(Debug, serde::Deserialize)]
struct Entry {
    output_start: i32,
    input_start: i32,
    input_range: i32,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::BufReader;

    use super::*;

    #[test]
    fn proper_deserialize_from_slice_to_entry() {
        // Note: must be without spaces
        let data = "\
output_start,input_start,input_range
50,98,2
52,50,48
";

        let mut rdr = Reader::from_reader(data.as_bytes());
        println!("Attempt to deserialize");
        for result in rdr.deserialize() {
            println!(" - Read one record");
            let record: Entry = result.unwrap();
            println!("{:?}", record);
        }
    }

    #[test]
    fn proper_deserialize_from_file_to_entry() {
        // Note: must be without spaces and by default the base directory should be at the same
        // level of src
        let file = File::open("content/sample-content.csv").unwrap();
        let reader = BufReader::new(file);

        let mut rdr = Reader::from_reader(reader);
        println!("Attempt to deserialize");
        for result in rdr.deserialize() {
            println!(" - Read one record");
            let record: Entry = result.unwrap();
            println!("{:?}", record);
        }
    }
}
