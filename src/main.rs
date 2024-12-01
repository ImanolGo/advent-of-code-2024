use std::env;
use anyhow::Result;

mod solutions;
mod utils;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Please provide the day number as an argument");
        println!("Usage: cargo run -- <day>");
        return Ok(());
    }

    let day = args[1].parse::<u8>()?;
    
    match day {
        1 => solutions::day01::solve()?,
        // Add more days here
        _ => println!("Day {} not implemented yet", day),
    }

    Ok(())
}