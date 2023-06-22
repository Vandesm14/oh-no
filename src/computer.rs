use core::fmt;

use petgraph::prelude::*;

pub type ComputerId = usize;
pub type MessagePort = u8;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
  pub port: MessagePort,
  pub edge: EdgeIndex,
  pub data: MessageData,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageData {
  BGPMessage { path: Vec<ComputerId> },
  Blank,
}

pub type MessageQueue = Vec<Message>;

// #[derive(Clone)]
// pub struct Computer {
//   pub id: ComputerID,
//   pub run: ComputerRun,
//   pub incoming: MessageQueue,
//   pub outgoing: MessageQueue,
// }

// impl fmt::Debug for Computer {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     let mut debugged = f.debug_struct("Computer");
//     debugged.field("id", &self.id);
//     debugged.field("incoming", &self.incoming);
//     debugged.field("outgoing", &self.outgoing);
//     debugged.finish()
//   }
// }

// impl Computer {
//   pub fn new(id: ComputerID, run: ComputerRun) -> Self {
//     Computer {
//       id,
//       run,
//       incoming: vec![],
//       outgoing: vec![],
//     }
//   }
// }

pub trait Computer: fmt::Debug {
  fn id(&self) -> ComputerId;
  fn id_mut(&mut self) -> &mut ComputerId;
  fn run(&mut self, edges: Vec<EdgeIndex>);

  fn incoming(&self) -> &MessageQueue;
  fn incoming_mut(&mut self) -> &mut MessageQueue;
  fn outgoing(&self) -> &MessageQueue;
  fn outgoing_mut(&mut self) -> &mut MessageQueue;
}

#[macro_export]
macro_rules! impl_computer_default {
  () => {
    fn id(&self) -> ComputerId {
      self.id
    }
    fn id_mut(&mut self) -> &mut ComputerId {
      &mut self.id
    }

    fn incoming(&self) -> &MessageQueue {
      &self.incoming
    }
    fn incoming_mut(&mut self) -> &mut MessageQueue {
      &mut self.incoming
    }
    fn outgoing(&self) -> &MessageQueue {
      &self.outgoing
    }
    fn outgoing_mut(&mut self) -> &mut MessageQueue {
      &mut self.outgoing
    }
  };
}