use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: u32,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContainer {
    pub messages: Vec<Message>,
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\r\n", self.content)
    }
}
