use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 8!");

    let args: Vec<String> = env::args().collect();
    let config = day8::Config::build(&args)?;
    let (unique_antinodes) = day8::run(config)?;

    println!("Num total unique_antinodes:         {unique_antinodes}");
    Ok(())
}
