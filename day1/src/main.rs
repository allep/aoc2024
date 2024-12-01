use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = day1::Config::build(&args)?;
    let total_distance = day1::run(config)?;

    println!("Total distance is: {total_distance}");
    Ok(())
}
