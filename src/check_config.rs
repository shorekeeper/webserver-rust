use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::log_info; // macro imports
use crate::log::info;
use crate::time::current_time;

const DEFAULT_CONFIG: &str = 
r#"SERVER_IP=your_ip
// !! LOCALHOST: 127.0.0.1:8080
// !! IP SHOULD BE DECLARED IN FORMAT IP:PORT
SMTP_USER=your_smtp_user
SMTP_PASS=your_smtp_pass
SMTP_HOST=your_smtp_host
DATABASE_URL=postgres://user:password@host/database"#;

pub fn check_config() {
    let now = current_time();
    let env_path = Path::new(".env");
    match env_path.exists() {
        false => {
            // create .env file with default values if it does not exist
            fs::write(env_path, DEFAULT_CONFIG).expect("Failed to create .env file");
            log_info!(&now, "Created .env file. Please configure it before running the program again.");
            thread::sleep(Duration::from_secs(10));
            std::process::exit(0);
        }
        true => {
            // check for missing values and add them to the file if necessary
            let mut config = fs::read_to_string(env_path).expect("Failed to read .env file");
            let mut updated = false;
            for line in DEFAULT_CONFIG.lines() {
                let key = line.split('=').next().unwrap();
                match config.contains(key) {
                    false => {
                        config.push_str("\n");
                        config.push_str(line);
                        updated = true;
                    }
                    true => {}
                }
            }
            match updated {
                true => {
                    fs::write(env_path, config).expect("Failed to update .env file");
                    log_info!(&now, "\x1B[1m\x1b[32mUpdated .env file with missing values. Please configure them before running the program again.\x1B[0m");
                    thread::sleep(Duration::from_secs(10));
                    std::process::exit(0);
                }
                false => {}
            }
        }
    }
}
