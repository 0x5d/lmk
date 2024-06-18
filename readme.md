# lmk

`lmk` is a command-line tool that monitors a CLI command and sends a Slack notification when the command finishes.

## How to run it

```sh
lmk --token $SLACK_BOT_TOKEN --channel $SLACK_CHANNEL_ID sleep 5
```

See **Setup** for more details.

## How to build it

```sh
cargo build
```

## Setup

### Obtaining a Slack HTTP API Bot Token

You'll need a Slack bot token so that `lmk` is able to authenticate with and use the Slack HTTP API. Follow their [guide](https://api.slack.com/tutorials/tracks/getting-a-token) to obtain one.

### Getting the Slack Channel ID

This is the ID of the channel you'd like `lmk` to post notifications to. After you've created an App for your organization (see **Obtaining a Slack HTTP API Bot Token**), invite your app to a channel or conversation and copy its ID. On a channel or conversation, click on the caret to the right of the title. A modal will pop up. The channel ID will be at the bottom.
