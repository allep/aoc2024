use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 9!");

    let args: Vec<String> = env::args().collect();
    let config = day9::Config::build(&args)?;
    let (checksum) = day9::run(config)?;

    println!("Checksum:         {checksum}");
    Ok(())
}
