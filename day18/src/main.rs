use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 18!");

    let args: Vec<String> = env::args().collect();
    let config = day18::Config::build(&args)?;
    let position = day18::run(config)?;

    println!(
        "First impossible position: ({}, {})",
        position.0, position.1
    );
    Ok(())
}
