use colored::Colorize;

pub fn error(now: &str, message: &str) {
    // define error function for printing error messages
    eprintln!("{} {} {}", now.bold().dimmed(), "[ERROR]".bold().red(), message.red()); // print formatted error message to stderr
}

pub fn warn(now: &str, message: &str) {
    // define warn function for printing warning messages
    eprintln!("{} {} {}", now.bold().dimmed(), "[WARN]".bold().yellow(), message.yellow()); // print formatted warning message to stderr
}

pub fn info(now: &str, message: &str) {
    // define info function for printing info messages
    println!("{} {} {}", now.bold().dimmed(), "[INFO]".bold().cyan(), message.cyan()); // print formatted info message to stdout
}

#[macro_export]
macro_rules! log_error {
    // define log_error macro for logging error messages
    ($now:expr, $($arg:tt)*) => {{
        error($now, &format!($($arg)*)); // call error function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_warn {
    // define log_warn macro for logging warning messages
    ($now:expr, $($arg:tt)*) => {{
        warn($now, &format!($($arg)*)); // call warn function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_info {
    // define log_info macro for logging info messages
    ($now:expr, $($arg:tt)*) => {{
        info($now, &format!($($arg)*)); // call info function with formatted arguments
    }}
}
