use crate::{
  impl_computer_default, Computer, ComputerId, Message, MessageData,
  MessageQueue,
};
use petgraph::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BgpComputer {
  id: ComputerId,
  incoming: MessageQueue,
  outgoing: MessageQueue,
}

impl Computer for BgpComputer {
  fn run(&mut self, edges: Vec<EdgeIndex>) {
    edges.into_iter().for_each(|edge| {
      self.outgoing.push(Message {
        port: 0,
        edge,
        data: MessageData::BGPMessage {
          path: vec![self.id],
        },
      })
    })
  }

  impl_computer_default!();
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CounterComputer {
  id: ComputerId,
  incoming: MessageQueue,
  outgoing: MessageQueue,
  counter: usize,
}

impl Computer for CounterComputer {
  fn run(&mut self, _edges: Vec<EdgeIndex>) {
    self.counter += self.incoming.len();
  }

  impl_computer_default!();
}