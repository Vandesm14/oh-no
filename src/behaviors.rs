use crate::{Computer, ComputerData, ComputerRunReturn, Message, MessageData};
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

/// Increments the counter for every message received
pub fn pc_count_up_on_receive(
  computer: &Computer,
  _edges: Vec<EdgeIndex>,
) -> ComputerRunReturn {
  println!("{}: {}", computer.id, computer.incoming.len());
  (
    vec![],
    Some(match computer.data {
      ComputerData::Counter(count) => {
        ComputerData::Counter(count + computer.incoming.len())
      }
      _ => computer.data.clone(),
    }),
  )
}
