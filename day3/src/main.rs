use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 3!");

    let args: Vec<String> = env::args().collect();
    let config = day3::Config::build(&args)?;
    let (total, total_filtered) = day3::run(config)?;

    println!("Total:          {total}");
    println!("Total filtered: {total_filtered}");
    Ok(())
}
