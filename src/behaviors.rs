use crate::{Computer, Message, MessageData, MessageQueue};
use petgraph::prelude::*;

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_bgp_msg(
  computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> MessageQueue {
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
) -> MessageQueue {
  edges
    .into_iter()
    .map(|edge| Message {
      port: 0,
      edge,
      data: MessageData::Blank,
    })
    .collect()
}
