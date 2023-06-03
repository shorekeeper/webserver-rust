use colored::Colorize;


pub fn error(now: &str, message: &str) {
    eprintln!("{} {} {}", now.bold().dimmed(), "[ERROR]".bold().red(), message.red());
}

pub fn warn(now: &str, message: &str) {
    eprintln!("{} {} {}", now.bold().dimmed(), "[WARN]".bold().yellow(), message.yellow());
}

pub fn info(now: &str, message: &str) {
    println!("{} {} {}", now.bold().dimmed(), "[INFO]".bold().cyan(), message.cyan());
}

#[macro_export]
macro_rules! log_error {
    ($now:expr, $($arg:tt)*) => {{
        error($now, &format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! log_warn {
    ($now:expr, $($arg:tt)*) => {{
        warn($now, &format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! log_info {
    ($now:expr, $($arg:tt)*) => {{
        info($now, &format!($($arg)*));
    }}
}

