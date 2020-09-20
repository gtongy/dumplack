use envy;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Error;
use std::process;
use std::process::{Command, Stdio};

#[derive(Deserialize, Debug)]
struct Config {
    user_name: String,
    password: String,
    host: String,
    port: u16,
    schema: String,
}

fn main() -> Result<(), Error> {
    let config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    const PARALLEL_NUM: u16 = 3;
    let outputs = File::create("output.sql")?;
    let errors = outputs.try_clone()?;
    Command::new("mysqlpump")
        .arg(format!("{}{}", "-u", config.user_name))
        .arg(format!("{}{}", "-p", config.password))
        .arg(format!("{}{}", "-h", config.host))
        .arg(format!("{} {}", "-P", config.port))
        .args(&["--single-transaction", "--skip-column-statistics"])
        .arg(format!("{}={}", "--default-parallelism", PARALLEL_NUM))
        .arg(format!("{}", config.schema))
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .status()
        .expect("failed to execute process");
    Ok(())
}
