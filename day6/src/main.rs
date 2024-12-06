use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 6!");

    let args: Vec<String> = env::args().collect();
    let config = day6::Config::build(&args)?;
    let (total, total_obstacles) = day6::run(config)?;

    println!("Num total positions:            {total}");
    println!("Num total obstacles positions:  {total_obstacles}");
    Ok(())
}
