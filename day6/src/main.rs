use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 6!");

    let args: Vec<String> = env::args().collect();
    let config = day6::Config::build(&args)?;
    let (total) = day6::run(config)?;

    println!("Num total positions:         {total}");
    Ok(())
}
