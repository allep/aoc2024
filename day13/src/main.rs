use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 13!");

    let args: Vec<String> = env::args().collect();
    let config = day13::Config::build(&args)?;
    let (total_cost) = day13::run(config)?;

    println!("Total cost:         {total_cost}");
    Ok(())
}
