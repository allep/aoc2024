use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, day 5!");

    let args: Vec<String> = env::args().collect();
    let config = day5::Config::build(&args)?;
    let (middle_page_sum, invalid_middle_page_sum) = day5::run(config)?;

    println!("Middle page sum:         {middle_page_sum}");
    println!("Invalid middle page sum: {invalid_middle_page_sum}");
    Ok(())
}
