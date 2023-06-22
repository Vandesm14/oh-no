pub type ComputerId = usize;
pub type MessagePort = u8;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  pub port: MessagePort,
  pub recipient_id: ComputerId,
  pub data: MessageData,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  BGPMessage {
    path: Vec<ComputerId>,
  },
  #[default]
  Blank,
}

pub type MessageQueue = Vec<Message>;
