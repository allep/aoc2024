use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 4!");

    let args: Vec<String> = env::args().collect();
    let config = day4::Config::build(&args)?;
    let total = day4::run(config)?;

    println!("Num total xmas:         {total}");
    Ok(())
}
