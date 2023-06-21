use oh_no::{Computer, Message, MessageData, World};
use petgraph::prelude::*;

fn pc_print_id(me: &mut Computer, edges: Vec<EdgeIndex>) {
  edges.into_iter().for_each(|edge| {
    me.queue_outgoing(
      edge,
      Message {
        port: 0,
        data: MessageData::BGPMessage { path: vec![me.id] },
      },
    )
  });
}

fn main() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_print_id);
  let pc2 = world.add_computer(pc_print_id);

  world.connect_computers(pc1, pc2);
  world.tick();

  println!("{:#?}", world);
  println!("{}", world)
}
