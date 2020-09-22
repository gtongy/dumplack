use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub schema: String,
    pub bucket_name: String,
    pub slack_hook: String,
    pub slack_channel_name: String,
    pub aws_region: String,
}
