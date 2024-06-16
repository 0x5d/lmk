use std::error::Error;

use clap::{arg, command, ArgAction};
use lmk::Lmk;
use slack::Client;

pub mod lmk;
pub mod slack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (token, channel, cmd, args) = parse_args();
    let client = Client::new(token.to_owned());

    let lmk = Lmk::new(client, channel.to_owned(), cmd.to_owned(), args);
    lmk.run().await
}

fn parse_args<'a>() -> (String, String, String, Vec<&'a str>) {
    let cmd_arg = arg!([CMD]);

    let args_arg = arg!([ARGS])
        .allow_hyphen_values(true)
        .action(ArgAction::Append)
        .trailing_var_arg(true);

    let token_arg = arg!(-t --token <TOKEN> "The Slack configuration token");
    let channel_arg = arg!(-c --channel <CHANNEL_ID> "The Slack channel ID to post notifications");

    let matches = command!()
        .arg(cmd_arg)
        .arg(args_arg)
        .arg(token_arg)
        .arg(channel_arg)
        .get_matches();

    let cmd = matches.get_one::<String>("CMD").expect("CMD is required");

    let args = matches
        .get_many::<String>("ARGS")
        .unwrap_or_default()
        .map(String::as_str)
        .collect::<Vec<_>>();

    let token = matches
        .get_one::<String>("token")
        .expect("--token is required");

    let channel = matches
        .get_one::<String>("channel")
        .expect("--channel is required");

    (token, channel, cmd, args)
}
