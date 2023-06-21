use std::collections::HashMap;

use oh_no::{
  queue_outgoing, Computer, Message, MessageData, MessageQueue, World,
};
use petgraph::prelude::*;

fn pc_print_id(computer: &Computer, edges: Vec<EdgeIndex>) -> MessageQueue {
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

#[test]
fn it_propagates_messages() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_print_id);
  let pc2 = world.add_computer(pc_print_id);

  world.connect_computers(pc1, pc2);
  world.tick();

  assert_eq!(
    world
      .get_computer(world.node_index_to_computer_id(pc1))
      .unwrap()
      .ingoing
      .len(),
    1
  );
}
