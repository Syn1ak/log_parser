use pest_derive::Parser;
use thiserror::Error;

/// Grammar parser for structured log entries.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

/// Error types for parsing log entries.
#[derive(Error, Debug)]
pub enum LogParseError {
    #[error("Failed to parse log entry")]
    ParsingError,
    #[error("Invalid date format")]
    DateFormatError,
    #[error("Invalid time format")]
    TimeFormatError,
    #[error("Invalid level format")]
    LevelFormatError,
    #[error("Invalid module format")]
    ModuleFormatError,
    #[error("Invalid request ID format")]
    RequestIDFormatError,
    #[error("Invalid error code format")]
    ErrorCodeFormatError,
}

/// Represents a four-digit year (e.g., "2024").
///
/// Format: `YYYY`
/// - `Y`: Any ASCII digit (0-9)
/// - Example: `"2024"`
pub const YEAR: &str = "year = { ASCII_DIGIT{4} }";

/// Represents a month with leading zero or "10" through "12" for valid month values.
///
/// Format: `MM`
/// - `"0"` followed by a single digit (0-9) or `"1"` followed by `0` to `2`.
/// - Example: `"01"` for January, `"12"` for December
pub const MONTH: &str = "month = { \"0\" ~ ASCII_DIGIT | \"1\" ~ '0'..'2' }";

/// Represents a day, allowing values from `01` to `31` based on the month’s possible days.
///
/// Format: `DD`
/// - Leading zeroes for single digits, `10` to `31`
/// - Example: `"01"`, `"31"`
pub const DAY: &str = "day = { \"0\" ~ ASCII_DIGIT | '1'..'2' ~ ASCII_DIGIT | \"3\" ~ '0'..'1' }";

/// Full date representation combining year, month, and day.
///
/// Format: `YYYY-MM-DD`
/// - Example: `"2024-11-06"`
pub const DATE: &str = "date = { year ~ \"-\" ~ month ~ \"-\" ~ day }";

/// Represents a time with an optional timezone.
///
/// Format: `HH:MM:SS(TZ)?`
/// - `TZ` is optional, e.g., `"UTC"`
/// - Example: `"12:00:00UTC"`
pub const TIME: &str = "time = { ASCII_DIGIT{2} ~ \":\" ~ ASCII_DIGIT{2} ~ \":\" ~ ASCII_DIGIT{2} ~ (ASCII_ALPHA{3})? }";

/// Represents a log level, supporting standard log levels.
///
/// Format: One of `"INFO"`, `"WARN"`, `"ERROR"`, `"DEBUG"`, `"TRACE"`, `"FATAL"`
/// - Example: `"INFO"`
pub const LEVEL: &str =
    "level = { \"INFO\" | \"WARN\" | \"ERROR\" | \"DEBUG\" | \"TRACE\" | \"FATAL\" }";

/// Represents a module, prefixed with `"mod-"` and followed by alphabetic characters.
///
/// Example: `"mod-network"`
pub const MODULE: &str = "module = { \"mod-\" ~ ASCII_ALPHA+ }";

/// Represents a request ID, prefixed with `"req-"` and followed by alphanumeric characters.
///
/// Example: `"req-12345"`
pub const REQUEST_ID: &str = "request_id = { \"req-\" ~ ASCII_ALPHANUMERIC+ }";

/// Represents an error code between 3 and 5 digits.
///
/// Example: `"404"`, `"12345"`
pub const ERROR_CODE: &str = "error_code = { ASCII_DIGIT{3,5} }";

/// Represents the log message, capturing all characters up to the end of the line.
///
/// Example: `"System started successfully"`
pub const MESSAGE: &str = "message = { (!\"\\n\" ~ ANY)+ }";

/// Represents a full log entry combining date, time, level, optional fields, and message.
///
/// Format: `YYYY-MM-DD HH:MM:SS Level [module] [request_id] [error_code] message`
/// - Example: `"2024-11-06 12:00:00UTC INFO System started successfully"`
pub const LOG_ENTRY: &str = "log_entry = { date ~ \" \" ~ time ~ \" \" ~ level ~ \" \" ~ (module ~ \" \")? ~ (request_id ~ \" \")? ~ (error_code ~ \" \")? ~ message }";
