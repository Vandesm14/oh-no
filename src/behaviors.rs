use crate::{
  impl_computer_default, Computer, ComputerId, Message, MessageData,
  MessageQueue,
};
use petgraph::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RomBgpComputer {
  id: ComputerId,
  incoming: MessageQueue,
  outgoing: MessageQueue,
}

impl Computer for RomBgpComputer {
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

// /// Sends a BGP propagation message to all neighbors
// pub fn pc_send_blank_msg(
//   _computer: &Computer,
//   edges: Vec<EdgeIndex>,
// ) -> ComputerRunReturn {
//   edges
//     .into_iter()
//     .map(|edge| Message {
//       port: 0,
//       edge,
//       data: MessageData::Blank,
//     })
//     .collect()
// }

// /// Increments the counter for every message received
// pub fn pc_count_up_on_receive(
//   computer: &Computer,
//   _edges: Vec<EdgeIndex>,
// ) -> ComputerRunReturn {
//   (
//     vec![],
//     if let Some(ComputerData::Counter(count)) = computer
//       .data
//       .iter()
//       .find(|data| matches!(data, ComputerData::Counter(_)))
//     {
//       HashSet::from([ComputerData::Counter(count + computer.incoming.len())])
//     } else {
//       HashSet::from([ComputerData::Counter(computer.incoming.len())])
//     },
//   )
// }
