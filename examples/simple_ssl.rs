extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let config = Config {
        nickname: Some("pickles".to_owned()),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#rust-spam".to_owned()]),
        use_ssl: Some(true),
        ..Default::default()
    };

    let client = IrcClient::from_config(config).unwrap();
    client.identify().unwrap();

    client.for_each_incoming(|message| {
        print!("{}", message);
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.contains("pickles") {
                client.send_privmsg(target, "Hi!").unwrap();
            }
        }
    }).unwrap();
}
