use anyhow::{Context, Result};
use clap::{Arg, Command};
use log_parser_by_syn1ak::{Grammar, Rule};
use pest::Parser;
use std::fs;
use std::process;

fn main() -> Result<()> {
    let matches = Command::new("Log Parser")
        .version("1.0")
        .author("o.syniak@ukma.edu.ua")
        .about("Parses structured log files with customizable fields")
        .arg(
            Arg::new("file")
                .help("The log file to parse")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(Command::new("credits").about("Displays credits information"))
        .get_matches();

    match matches.subcommand() {
        Some(("credits", _)) => {
            println!("Log Parser 1.0 by o.syniak@ukma.edu.ua");
            println!("Developed to parse structured log files for analysis (using Pest parsing library).");
            return Ok(());
        }
        _ => {
            if let Some(file_path) = matches.get_one::<String>("file") {
                if let Err(err) = parse_file(file_path) {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            } else {
                println!("Usage: log_parser <file_path> or log_parser credits");
                process::exit(1);
            }
        }
    }

    Ok(())
}

fn parse_file(file_path: &str) -> Result<()> {
    let log_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    for line in log_content.lines() {
        let parsed = Grammar::parse(Rule::log_entry, line)
            .with_context(|| format!("Failed to parse log entry: {}", line))?;

        for pair in parsed {
            println!("Parsed entry: {:?}", pair);
        }
    }
    Ok(())
}
