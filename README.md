# Server session log analyzer
This is a tool written to parse CSV log files in the format

| session\_id | access timestamp |
| ----------- | ---------------- |
| <entry 1>   | <entry 2>        |

## Installation
To build this project, use the `cargo` rust build system.
- `cargo run <CSV Logfile> -d <YY-mm-dd>` get most frequent session accesses for a given date
- `cargo run --help` get CLI help with program
- `cargo test` run unit tests
- `cargo build` build optimized version of code

