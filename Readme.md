# log_parser

## Overview
`log_parser` is a Rust project designed to parse structured log files. It is built using the Pest parsing library, which enables the extraction of specific components from log entries, such as date, time, log level, and message content.

## Parsing Process
The parsing process is centered around a simple grammar that defines the structure of each log entry. Each entry is expected to be in the format:

### Grammar Components
- **Date**: Recognizes dates in the format `YYYY-MM-DD`.
- **Time**: Recognizes times in the format `HH:MM:SS`.
- **Level**: Recognizes standard log levels like `INFO`, `WARN`, `ERROR`, and `DEBUG`.
- **Message**: The log message, which includes any textual information following the log level.

### Usage
By parsing logs in this structured way, applications can perform detailed log analysis and visualization, improving error tracking and debugging processes.