use std::{error::Error, process::Command};

use crate::slack::{Client, PostMessageReq};

pub struct Lmk<'a> {
    client: Client,
    channel: String,
    cmd: String,
    args: Vec<&'a str>,
}

impl<'a> Lmk<'a> {
    pub fn new(client: Client, channel: String, cmd: String, args: Vec<&'a str>) -> Self {
        Lmk {
            client,
            channel,
            cmd,
            args,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let out = Command::new(self.cmd.as_str())
            .args(self.args.as_slice())
            .output()
            .expect("failed to execute process");
        let exit_code = out.status.code().unwrap_or(0);
        if exit_code != 0 {
            // TODO: print stderr contents.
            let message = format!(
                "Command `{}` returned non-zero code {}.",
                self.cmd, out.status,
            );
            return self.post_slack_message(message.clone()).await;
        }
        // TODO: print stdout contents.
        self.post_slack_message(format!("Command `{}` finished.", self.cmd))
            .await
    }

    async fn post_slack_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        let req = PostMessageReq {
            channel: self.channel.clone(),
            text: message,
        };
        self.client.post_message(req).await.map(|_| ())
    }
}
