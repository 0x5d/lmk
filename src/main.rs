use clap::{arg, command, ArgAction};

pub mod slack;

fn main() {
    let cmd_arg = arg!([CMD])
        .allow_hyphen_values(true)
        .action(ArgAction::Append)
        .trailing_var_arg(true);

    let token_arg = arg!(-t --token <TOKEN> "The Slack configuration token");

    let matches = command!().arg(cmd_arg).arg(token_arg).get_matches();

    let cmd = matches
        .get_many::<String>("CMD")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    println!("{:#?}", cmd);

    let token = matches
        .get_one::<String>("token")
        .expect("--token is required");
    println!("{:?}", token);
}
