use pest::Parser;
use anyhow::{Result, anyhow};
use log_parser::*;

#[test]
fn test_valid_log_entry() -> Result<()> {
    let log_entry = "2024-11-06 12:00:00 INFO System started successfully";
    let pair = Grammar::parse(Rule::log_entry, log_entry)?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse valid log entry"))?;

    assert_eq!(pair.as_str(), log_entry);

    let mut components = pair.into_inner();
    assert_eq!(components.next().unwrap().as_str(), "2024-11-06"); 
    assert_eq!(components.next().unwrap().as_str(), "12:00:00"); 
    assert_eq!(components.next().unwrap().as_str(), "INFO");
    assert_eq!(components.next().unwrap().as_str(), "System started successfully");

    Ok(())
}

#[test]
fn test_missing_message() {
    let log_entry_missing_message = "2024-11-06 12:34:56 INFO";
    let result = Grammar::parse(Rule::log_entry, log_entry_missing_message);
    assert!(result.is_err(), "Fail when the message part is missing");
}
