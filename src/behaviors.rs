use std::collections::HashMap;

use crate::{queue_outgoing, Computer, Message, MessageData, MessageQueue};
use petgraph::prelude::*;

/// Sends a BGP propagation message to all neighbors
pub fn pc_send_bgp_msg(
  computer: &Computer,
  edges: Vec<EdgeIndex>,
) -> MessageQueue {
  let mut message_queue: MessageQueue = HashMap::new();
  edges.into_iter().for_each(|edge| {
    queue_outgoing(
      &mut message_queue,
      edge,
      Message {
        port: 0,
        data: MessageData::BGPMessage {
          path: vec![computer.id],
        },
      },
    )
  });

  message_queue
}
