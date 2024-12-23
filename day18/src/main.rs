use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 18!");

    let args: Vec<String> = env::args().collect();
    let config = day18::Config::build(&args)?;
    let (min) = day18::run(config)?;

    println!("Min:         {min}");
    Ok(())
}
