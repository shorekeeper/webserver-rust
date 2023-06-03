use chrono::Local;

// wtf why tf should i even put this in separate files rust???
pub fn current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}