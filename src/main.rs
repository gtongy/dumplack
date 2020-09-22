use bytes::Bytes;
use chrono::Utc;
use envy;
use futures::{FutureExt, TryStreamExt};
use rusoto_core::Region;
use rusoto_s3::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::process;
use std::process::{Command, Stdio};

mod config;
mod slack;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = match envy::from_env::<config::Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let slack = slack::SlackClient::new(config.slack_hook, config.slack_channel_name);
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
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    // TODO: stderrのエラーハンドリング

    if let Some(_du_output) = sql_file_output.stdout.take() {
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let output_file_name = format!("{}.sql", now);
        let outputs = File::create(output_file_name.clone())?;
        Command::new("masking")
            .stdin(_du_output)
            .stdout(Stdio::from(outputs))
            .spawn()?;

        let meta = std::fs::metadata(output_file_name.clone()).unwrap();
        let read_stream = tokio::fs::read(output_file_name.clone().to_owned())
            .into_stream()
            .map_ok(Bytes::from);
        let req = PutObjectRequest {
            bucket: config.bucket_name.clone(),
            key: String::from(output_file_name.clone()),
            content_length: Some(meta.len() as i64),
            body: Some(StreamingBody::new(read_stream)),
            ..Default::default()
        };
        let s3_client = S3Client::new(Region::default());
        s3_client.put_object(req).await.expect("");
        let url = format!(
            "https://{}.s3-{}.amazonaws.com/{}",
            &config.bucket_name,
            &config.aws_region,
            output_file_name.clone()
        );
        slack
            .notify_send_file_url(String::from("file download link get!\n"), url)
            .await;
    }
    Ok(())
}
