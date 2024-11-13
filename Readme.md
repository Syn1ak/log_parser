# log_parser_by_syn1ak

## Overview

`log_parser_by_syn1ak` is a Rust project designed to parse structured log files. It is built using the Pest parsing library, which enables the extraction of specific components from log entries, such as date, time, log level, and message content.

## Parsing Process

The parser works by defining each component of a log entry through distinct grammar rules in Pest, allowing it to identify and extract specific parts of a log. This design enables you to capture structured data from log files, which can be used in log analysis pipelines, monitoring systems, and debugging processes.

### Grammar Components

Each log entry has the following structure:

```
log_entry = { date ~ " " ~ time ~ " " ~ level ~ " " ~ (module ~ " ")? ~ (request_id ~ " ")? ~ (error_code ~ " ")? ~ message }
```

- **Date**: A date in `YYYY-MM-DD` format.
- **Time**: A time in `HH:MM:SS` format, optionally followed by a three-letter timezone (e.g., `UTC`).
- **Level**: The severity level of the log (e.g., `INFO`, `WARN`, `ERROR`, `DEBUG`).
- **Module**: An optional module field prefixed with `mod-`, representing the module or subsystem generating the log.
- **Request ID**: An optional identifier prefixed with `req-`, allowing tracking of requests or processes.
- **Error Code**: An optional numeric error code with 3-5 digits, used to indicate specific error types.
- **Message**: The main log message content, capturing everything up to the end of the line.

### Grammar Rule Diagram

Here’s a breakdown of each grammar rule used in `log_parser_by_syn1ak`:

- **Year**: `year = { ASCII_DIGIT{4} }`
- **Month**: `month = { "0" ~ ASCII_DIGIT | "1" ~ '0'..'2' }`
- **Day**: `day = { "0" ~ ASCII_DIGIT | '1'..'2' ~ ASCII_DIGIT | "3" ~ '0'..'1' }`
- **Date**: `date = { year ~ "-" ~ month ~ "-" ~ day }`
- **Time**: `time = { ASCII_DIGIT{2} ~ ":" ~ ASCII_DIGIT{2} ~ ":" ~ ASCII_DIGIT{2} ~ (ASCII_ALPHA{3})? }`
- **Level**: `level = { "INFO" | "WARN" | "ERROR" | "DEBUG" | "TRACE" | "FATAL" }`
- **Module**: `module = { "mod-" ~ ASCII_ALPHA+ }`
- **Request ID**: `request_id = { "req-" ~ ASCII_ALPHANUMERIC+ }`
- **Error Code**: `error_code = { ASCII_DIGIT{3,5} }`
- **Message**: `message = { (!"\n" ~ ANY)+ }`

### Example Log Entry

An example log entry that follows this structure might look like:

```
2024-11-06 12:00:00UTC INFO mod-network req-12345 404 System started successfully
```

### Usage

By parsing logs in this structured way, applications can perform detailed log analysis and visualization, improving error tracking and debugging processes.

1. **Parsing a Log File**:
   To parse a log file, specify the file path:

   ```bash
   cargo run -- <file_path>
   ```

   or

   ```bash
   make run FILE=<file_path>
   ```

2. **Display Credits**:
   Use the `credits` subcommand to display project information:

   ```bash
   cargo run -- credits
   ```

   or

   ```bash
   make credits
   ```

3. **Run Tests**:
   To test the parser and verify the grammar rules, run:
   ```bash
   cargo test
   ```
   or
   ```bash
   make test
   ```

### Documentation and Links

- **Crates.io**: [Link to crates.io page](https://crates.io/crates/log_parser_by_syn1ak)
- **Docs.rs**: [Link to docs.rs documentation](https://docs.rs/log_parser_by_syn1ak/latest/log_parser_by_syn1ak/)
