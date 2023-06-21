use crate::{Computer, ComputerRunReturn, Message, MessageData};
use petgraph::prelude::*;

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_bgp_msg(
  computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> ComputerRunReturn {
  (
    edges
      .into_iter()
      .map(|edge| Message {
        port: 0,
        edge,
        data: MessageData::BGPMessage {
          path: vec![computer.id],
        },
      })
      .collect(),
    None,
  )
}

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_blank_msg(
  _computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> ComputerRunReturn {
  (
    edges
      .into_iter()
      .map(|edge| Message {
        port: 0,
        edge,
        data: MessageData::Blank,
      })
      .collect(),
    None,
  )
}
