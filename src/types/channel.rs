use std::fmt::{self, Display};

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: u32,
    pub name: String,
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\r\n ", self.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelContainer {
    pub channels: Vec<Channel>,
}

pub fn find_channel_by_name<'a>(channels: &'a [Channel], name: &str) -> Option<&'a Channel> {
    for channel in channels {
        if channel.name == name {
            return Some(channel);
        }
    }
    None
}
