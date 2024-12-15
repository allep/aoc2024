use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 15!");

    let args: Vec<String> = env::args().collect();
    let config = day15::Config::build(&args)?;
    let (sum) = day15::run(config)?;

    println!("Total sum:         {sum}");
    Ok(())
}
