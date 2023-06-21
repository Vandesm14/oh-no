use petgraph::prelude::*;
use std::{
  collections::{hash_map::Entry, HashMap},
  fmt,
};

pub type ComputerID = usize;
pub type ComputerRun = fn(&mut Computer, Vec<EdgeIndex>);
pub type MessagePort = u8;

#[derive(Debug)]
pub struct Message {
  pub port: MessagePort,
  pub data: MessageData,
}

#[derive(Debug)]
pub enum MessageData {
  BGPMessage { path: Vec<ComputerID> },
}

pub struct Computer {
  pub id: ComputerID,
  pub run: ComputerRun,
  pub ingoing: Vec<Message>,
  pub outgoing: HashMap<EdgeIndex, Vec<Message>>,
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
      ingoing: vec![],
      outgoing: HashMap::new(),
    }
  }

  pub fn queue_outgoing(&mut self, via_edge: EdgeIndex, message: Message) {
    match self.outgoing.entry(via_edge) {
      Entry::Occupied(mut entry) => {
        entry.get_mut().push(message);
      }
      Entry::Vacant(_) => {
        self.outgoing.insert(via_edge, vec![message]);
      }
    }
  }
}
