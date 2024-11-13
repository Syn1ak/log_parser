use anyhow::{anyhow, Result};
use log_parser_by_syn1ak::{Grammar, LogParseError, Rule};
use pest::Parser;

#[test]
fn test_valid_log_entry() -> Result<()> {
    let log_entry = "2024-11-06 12:00:00UTC INFO System started successfully";
    let mut parsed =
        Grammar::parse(Rule::log_entry, log_entry).map_err(|_| LogParseError::ParsingError)?;

    let mut components = parsed
        .next()
        .ok_or_else(|| anyhow!("Failed to parse valid log entry"))?
        .into_inner();

    assert_eq!(components.next().unwrap().as_str(), "2024-11-06");
    assert_eq!(components.next().unwrap().as_str(), "12:00:00UTC");
    assert_eq!(components.next().unwrap().as_str(), "INFO");
    assert_eq!(
        components.next().unwrap().as_str(),
        "System started successfully"
    );

    Ok(())
}

#[test]
fn test_invalid_date() {
    let log_entry = "2024-13-40 12:00:00UTC INFO Invalid date format";

    let result = Grammar::parse(Rule::date, log_entry);

    assert!(
        result.is_err(),
        "Expected date parsing to fail with invalid date format"
    );
}

#[test]
fn test_invalid_time() {
    let log_entry = "2024-11-06 25:61:61UTC INFO Invalid time format";

    let result = Grammar::parse(Rule::time, log_entry);

    assert!(
        result.is_err(),
        "Expected time parsing to fail with invalid time format"
    );
}

#[test]
fn test_invalid_level() {
    let log_entry = "2024-11-06 12:00:00UTC UNKNOWN_LEVEL Invalid level format";

    let result = Grammar::parse(Rule::level, log_entry);

    assert!(
        result.is_err(),
        "Expected level parsing to fail with invalid level format"
    );
}

#[test]
fn test_invalid_request_id() {
    let log_entry = "req-!@#";

    let result = Grammar::parse(Rule::request_id, log_entry);

    assert!(
        result.is_err(),
        "Expected request ID parsing to fail with invalid characters"
    );
}

#[test]
fn test_invalid_error_code() {
    let log_entry = "error_code_12345";

    let result = Grammar::parse(Rule::error_code, log_entry);

    assert!(
        result.is_err(),
        "Expected error code parsing to fail with invalid format"
    );
}

#[test]
fn test_missing_message() -> Result<()> {
    let log_entry = "2024-11-06 12:34:56UTC INFO";

    let result =
        Grammar::parse(Rule::log_entry, log_entry).map_err(|_| LogParseError::ParsingError);

    assert!(
        result.is_err(),
        "Expected log entry parsing to fail when message is missing"
    );

    Ok(())
}

#[test]
fn test_valid_module() -> Result<()> {
    let module = "mod-network";
    let mut parsed =
        Grammar::parse(Rule::module, module).map_err(|_| LogParseError::ParsingError)?;

    let parsed_module = parsed
        .next()
        .ok_or_else(|| anyhow!("Failed to parse valid module"))?;
    assert_eq!(parsed_module.as_str(), module);

    Ok(())
}
