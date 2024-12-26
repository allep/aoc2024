use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 19!");

    let args: Vec<String> = env::args().collect();
    let config = day19::Config::build(&args)?;
    let num_possible = day19::run(config)?;

    println!("Num possible designs: {num_possible}");

    Ok(())
}
