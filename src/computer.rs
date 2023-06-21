use petgraph::prelude::*;
use std::fmt;

pub type ComputerID = usize;
pub type ComputerRun = fn(computer: &Computer, Vec<EdgeIndex>) -> MessageQueue;
pub type MessagePort = u8;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  pub port: MessagePort,
  pub edge: EdgeIndex,
  pub data: MessageData,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  BGPMessage { path: Vec<ComputerID> },
  Blank,
}

pub type MessageQueue = Vec<Message>;

#[derive(Clone)]
pub struct Computer {
  pub id: ComputerID,
  pub run: ComputerRun,
  pub incoming: MessageQueue,
  pub outgoing: MessageQueue,
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut debugged = f.debug_struct("Computer");
    debugged.field("id", &self.id);
    debugged.field("incoming", &self.incoming);
    debugged.field("outgoing", &self.outgoing);
    debugged.finish()
  }
}

impl Computer {
  pub fn new(id: ComputerID, run: ComputerRun) -> Self {
    Computer {
      id,
      run,
      incoming: vec![],
      outgoing: vec![],
    }
  }
}
