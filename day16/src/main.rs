use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 16!");

    let args: Vec<String> = env::args().collect();
    let config = day16::Config::build(&args)?;
    let (score, num) = day16::run(config)?;

    println!("Total score: {score}\nTotal num: {num}");
    Ok(())
}
