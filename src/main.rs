use envy;
use serde_derive::Deserialize;
use std::process;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    user_name: String,
    password: String,
    host: String,
    port: u16,
    schema: String,
}

fn main() {
    let config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    const PARALLEL_NUM: u16 = 3;
    const FILENAME: &str = "demo.sql";
    Command::new("mysqlpump")
        .arg(format!("{}{}", "-u", config.user_name))
        .arg(format!("{}{}", "-p", config.password))
        .arg(format!("{}{}", "-h", config.host))
        .arg(format!("{} {}", "-P", config.port))
        .args(&["--single-transaction", "--skip-column-statistics"])
        .arg(format!("{}={}", "--default-parallelism", PARALLEL_NUM))
        .arg(format!("{}", config.schema))
        .arg(format!("> {}", FILENAME))
        .status()
        .expect("failed to execute process");
}
