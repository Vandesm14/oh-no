use std::collections::HashSet;

use crate::{Computer, ComputerRunReturn, Message, MessageData};
use petgraph::prelude::*;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum ComputerData {
//   Blank,
//   Counter(usize),
// }

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_bgp_msg(
  computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> ComputerRunReturn {
  edges
    .into_iter()
    .map(|edge| Message {
      port: 0,
      edge,
      data: MessageData::BGPMessage {
        path: vec![computer.id],
      },
    })
    .collect()
}

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_blank_msg(
  _computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> ComputerRunReturn {
  edges
    .into_iter()
    .map(|edge| Message {
      port: 0,
      edge,
      data: MessageData::Blank,
    })
    .collect()
}

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
