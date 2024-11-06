use pest::Parser;
use anyhow::{anyhow, Result};
use log_parser::*;

fn main() -> Result<()> {
    let log_entry = "2024-11-06 12:34:56 INFO System started successfully";
    let pair = Grammar::parse(Rule::log_entry, log_entry)?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse log entry"))?;

    println!("Parsed log entry: {:?}", pair);

    Ok(())
}