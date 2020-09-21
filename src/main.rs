use bytes::Bytes;
use envy;
use futures::{FutureExt, TryStreamExt};
use rusoto_core::Region;
use rusoto_s3::*;
use serde_derive::Deserialize;
use slack_hook::SlackTextContent::Text;
use slack_hook::{PayloadBuilder, Slack};
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
    bucket_name: String,
    slack_hook: String,
    slack_channel_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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
        let meta = std::fs::metadata(OUTPUT_FILE_NAME).unwrap();
        let read_stream = tokio::fs::read(OUTPUT_FILE_NAME.to_owned())
            .into_stream()
            .map_ok(Bytes::from);
        let req = PutObjectRequest {
            bucket: config.bucket_name.clone(),
            key: String::from(OUTPUT_FILE_NAME),
            content_length: Some(meta.len() as i64),
            body: Some(StreamingBody::new(read_stream)),
            ..Default::default()
        };
        let s3_client = S3Client::new(Region::ApNortheast1);
        s3_client
            .put_object(req)
            .await
            .expect("Couldn't PUT object");
        let slack = Slack::new(config.slack_hook.as_ref()).unwrap();
        let url = format!(
            "https://{}.s3-ap-northeast-1.amazonaws.com/{}",
            &config.bucket_name, OUTPUT_FILE_NAME
        );
        let p = PayloadBuilder::new()
            .text(vec![Text("file download link get!\n".into()), Text(url.into())].as_slice())
            .channel(config.slack_channel_name)
            .build()
            .unwrap();

        let res = slack.send(&p);
        match res {
            Ok(()) => println!("ok"),
            Err(x) => println!("ERR: {:?}", x),
        }
    }
    Ok(())
}
