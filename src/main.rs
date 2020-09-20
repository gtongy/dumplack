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
    let mut sql_file_output = Command::new("mysqldump")
        .arg(format!("{}{}", "-u", config.user_name))
        .arg(format!("{}{}", "-p", config.password))
        .arg(format!("{}{}", "-h", config.host))
        .arg(format!("{} {}", "-P", config.port))
        .args(&[
            "--single-transaction",
            "--skip-column-statistics",
            "--complete-insert",
        ])
        .arg(format!("{}", config.schema))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    if let Some(_du_output) = sql_file_output.stdout.take() {
        const OUTPUT_FILE_NAME: &str = "output.sql";
        let outputs = File::create(OUTPUT_FILE_NAME)?;
        let errors = outputs.try_clone()?;
        Command::new("masking")
            .stdin(_du_output)
            .stdout(Stdio::piped())
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .spawn()?;
        sql_file_output.wait()?;
    }
    Ok(())
}
