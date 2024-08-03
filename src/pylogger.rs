use pyo3::prelude::FromPyObject;

#[derive(FromPyObject)]
pub enum PrintableObject {
    IntNum(usize),
    FloatNum(f64),
    PrintableObj(String),
}

pub struct LogEntry {
    prefix_str: String,
    args: Vec<PrintableObject>,
}

pub fn new(prefix_str: &str, args: Vec<PrintableObject>) -> LogEntry {
    LogEntry {
        prefix_str: prefix_str.to_string(),
        args,
    }
}

pub fn log_to_console(log_entry: &LogEntry) {
    let mut log_msg = String::new();
    log_msg += &(format!("{}", log_entry.prefix_str).to_string());
    for arg in &log_entry.args {
        match arg {
            PrintableObject::IntNum(v) => {
                log_msg += &(format!(" {}", v).to_string());
            }
            PrintableObject::FloatNum(v) => {
                log_msg += &(format!(" float: {}", v).to_string());
            }
            PrintableObject::PrintableObj(v) => {
                log_msg += &(format!(" str: {}", v).to_string());
            }
        }
    }
    println!("{}", log_msg);
}
