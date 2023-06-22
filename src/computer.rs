pub type MessagePort = u8;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  pub port: MessagePort,
  pub recipient_id: usize,
  pub data: MessageData,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  #[default]
  Blank,
  BGPMessage {
    path: Vec<usize>,
  },
}

pub type MessageQueue = Vec<Message>;
