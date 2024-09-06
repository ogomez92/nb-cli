use std::fmt::{self, Display};

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    messages: Vec<String>,
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.id, self.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelContainer {
    pub channels: Vec<Channel>,
}
