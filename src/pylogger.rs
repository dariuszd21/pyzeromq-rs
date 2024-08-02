use std::fmt::Display;

pub struct LogEntry {
    log_entry: String,
}

pub fn init(log_entry: &str) -> LogEntry {
    LogEntry {
        log_entry: log_entry.to_string(),
    }
}

pub fn log(fmt_str: &str) {
    println!("{}", fmt_str);
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.log_entry)
    }
}
