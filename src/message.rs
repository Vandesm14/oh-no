/// A message port
pub type MessagePort = u8;

/// A network message
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  /// The port that the message was sent from
  pub port: MessagePort,
  /// The ID of the recipient (Computer ID)
  pub recipient_id: usize,
  /// The data of the message
  pub data: MessageData,
}

/// The data of a message
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  #[default]
  Blank,
  BGPMessage {
    path: Vec<usize>,
  },
}

/// A queue of messages
pub type MessageQueue = Vec<Message>;
