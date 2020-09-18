use envy;
use serde_derive::Deserialize;
use std::process;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    user: String,
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
    println!("{:#?}", config);
    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .spawn()
        .expect("failed to execute process");
}
