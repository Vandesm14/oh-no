use petgraph::prelude::*;
use std::{
  collections::{hash_map::Entry, HashMap},
  fmt,
};

pub type ComputerID = usize;
pub type ComputerRun = fn(computer: &Computer, Vec<EdgeIndex>) -> MessageQueue;
pub type MessagePort = u8;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  pub port: MessagePort,
  pub data: MessageData,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  BGPMessage { path: Vec<ComputerID> },
}

pub type MessageQueue = HashMap<EdgeIndex, Vec<Message>>;

pub struct Computer {
  pub id: ComputerID,
  pub run: ComputerRun,
  pub ingoing: MessageQueue,
  pub outgoing: MessageQueue,
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut debugged = f.debug_struct("Computer");
    debugged.field("id", &self.id);
    debugged.field("ingoing", &self.ingoing);
    debugged.field("outgoing", &self.outgoing);
    debugged.finish()
  }
}

impl Computer {
  pub fn new(id: ComputerID, run: ComputerRun) -> Self {
    Computer {
      id,
      run,
      ingoing: HashMap::new(),
      outgoing: HashMap::new(),
    }
  }
}

pub fn queue_outgoing(
  queue: &mut MessageQueue,
  via_edge: EdgeIndex,
  message: Message,
) {
  match queue.entry(via_edge) {
    Entry::Occupied(mut entry) => {
      entry.get_mut().push(message);
    }
    Entry::Vacant(_) => {
      queue.insert(via_edge, vec![message]);
    }
  }
}
