use slack_hook::SlackTextContent::Text;
use slack_hook::{PayloadBuilder, Slack};

#[derive(Debug)]
pub struct SlackClient {
    channel: String,
    hook: String,
    ins: Slack,
}

impl SlackClient {
    pub fn new(hook: String, channel: String) -> Self {
        let slack = Slack::new(hook.as_ref()).unwrap();
        SlackClient {
            channel: channel,
            hook: hook,
            ins: slack,
        }
    }

    pub async fn notify_send_file_url(&self, text: String, url: String) {
        let p = PayloadBuilder::new()
            .text(vec![Text(text.into()), Text(url.into())].as_slice())
            .channel(self.channel.clone())
            .build()
            .unwrap();
        let res = self.ins.send(&p);
        match res {
            Ok(()) => println!("ok"),
            Err(x) => println!("ERR: {:?}", x),
        }
    }
}
