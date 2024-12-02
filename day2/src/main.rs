use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 2!");

    let args: Vec<String> = env::args().collect();
    let config = day2::Config::build(&args)?;
    let (num_safe, num_loosely_safe) = day2::run(config)?;

    println!("Num safe:         {num_safe}");
    println!("Num loosely safe: {num_loosely_safe}");
    Ok(())
}
