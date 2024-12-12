use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 10!");

    let args: Vec<String> = env::args().collect();
    let config = day10::Config::build(&args)?;
    let (result) = day10::run(config)?;

    println!("Total sum of scores:         {result}");
    Ok(())
}
