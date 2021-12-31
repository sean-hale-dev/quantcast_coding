use chrono::{DateTime, Datelike, FixedOffset, NaiveDate};
use clap::Parser;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
/// A tool for analyzing cookie logs for usage frequency
struct CliArgs {
    /// The log file to analyze
    log_file: String,

    #[clap(short)]
    /// The date (UTC format) to search for frequencies
    date: String,
}

struct LogEntry {
    cookie: String,
    timestamp: DateTime<FixedOffset>,
}

impl LogEntry {
    fn new(entry: &str) -> Self {
        let (cookie, timestamp) = entry.split_once(',').expect("Improper CSV row");

        let timestamp = DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%S%z")
            .expect("Unable to parse cookie timestamp");
        let cookie = cookie.to_string();

        LogEntry { cookie, timestamp }
    }
}

fn main() {
    let args = CliArgs::parse();

    let filter_date =
        NaiveDate::parse_from_str(&args.date, "%Y-%m-%d").expect("Unable to parse filter date");

    let entries = parse_logs(&args.log_file);
    let frequencies = get_max_freq_date(&entries, &filter_date);

    for max_freq_cookie in frequencies {
        println!("{}", max_freq_cookie);
    }
}

fn parse_logs(input: &str) -> Vec<LogEntry> {
    let log_file =
        read_to_string(input).expect(&format!("Unable to open logfile {}", input).to_string());

    let mut entries = Vec::new();
    for line in log_file.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        entries.push(LogEntry::new(&line));
    }

    entries
}

fn get_max_freq_date(cookie_stamps: &Vec<LogEntry>, filter_date: &NaiveDate) -> Vec<String> {
    let mut cookie_map: HashMap<String, usize> = HashMap::new();
    let mut max_hits = usize::MIN;

    for entry in cookie_stamps.iter().filter(|elem| {
        elem.timestamp.year() == filter_date.year()
            && elem.timestamp.month() == filter_date.month()
            && elem.timestamp.day() == filter_date.day()
    }) {
        let e = cookie_map.entry(entry.cookie.clone()).or_default();
        *e += 1;
        max_hits = (*e).max(max_hits);
    }

    cookie_map
        .into_iter()
        .filter(|(_, count)| count == &max_hits)
        .map(|(cookie, _)| cookie)
        .collect()
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    #[should_panic]
    fn invalid_logfile() {
        let _entries = super::parse_logs("i_dont_exist.csv");
    }

    #[test]
    fn empty_logfile() {
        let empty_file_entries = super::parse_logs("tests/empty_log.csv");
        let only_header_entries = super::parse_logs("tests/only_heading_log.csv");

        assert!(empty_file_entries.is_empty());
        assert!(only_header_entries.is_empty());
    }

    #[test]
    #[should_panic]
    fn unparseable_logfile() {
        let _entries = super::parse_logs("tests/invalid_timestamps.csv");
    }

    #[test]
    fn valid_logfile() {
        let entries = super::parse_logs("tests/valid_log.csv");

        assert_eq!(entries.len(), 8);
    }

    #[test]
    fn no_max_frequencies() {
        let empty_file_entries = super::parse_logs("tests/empty_log.csv");
        let max_freq =
            super::get_max_freq_date(&empty_file_entries, &NaiveDate::from_ymd(2018, 2, 14));

        assert_eq!(max_freq.len(), 0);
    }

    #[test]
    fn provided_sample_max_frequencies() {
        let entries = super::parse_logs("tests/valid_log.csv");

        let max_frequencies_single =
            super::get_max_freq_date(&entries, &NaiveDate::from_ymd(2018, 12, 9));

        let max_frequencies_multiple =
            super::get_max_freq_date(&entries, &NaiveDate::from_ymd(2018, 12, 8));

        assert_eq!(max_frequencies_single.len(), 1);
        assert_eq!(max_frequencies_multiple.len(), 3);
    }
}
