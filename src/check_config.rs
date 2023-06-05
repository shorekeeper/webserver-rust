use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use logger_rust::*;

const DEFAULT_CONFIG: &str = 
r#"# !! LOCALHOST: 127.0.0.1:8080
# !! IP SHOULD BE DECLARED IN FORMAT IP:PORT
SERVER_IP=your_ip
SMTP_USER=your_smtp_user
SMTP_PASS=your_smtp_pass
SMTP_HOST=your_smtp_host
DATABASE_URL=postgres://user:password@host/database"#;

pub fn check_config() -> Result<(), std::io::Error> {
    let env_path = Path::new(".env");
    match env_path.exists() {
        false => {
            // create .env file with default values if it does not exist
            fs::write(env_path, DEFAULT_CONFIG)?;
            log_info!("Created .env file. Please configure it before running the program again.");
            thread::sleep(Duration::from_secs(10));
            std::process::exit(0);
        }
        true => {
            // check for missing values and add them to the file if necessary
            let mut config = fs::read_to_string(env_path)?;
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
                    fs::write(env_path, config)?;
                    log_info!("\x1B[1m\x1b[32mUpdated .env file with missing values. Please configure them before running the program again.\x1B[0m");
                    thread::sleep(Duration::from_secs(10));
                    std::process::exit(0);
                }
                false => {}
            }
        }
    }

    Ok(())
}