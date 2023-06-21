use oh_no::{pc_send_blank_msg, World};

#[test]
fn it_propagates_messages() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_blank_msg);
  let pc2 = world.add_computer(pc_send_blank_msg);

  world.connect_computers(pc1.node, pc2.node);
  world.tick();

  // PC1 should have a message in its incoming queue, but not outgoing
  assert_eq!(world.get_computer(pc1.id).unwrap().ingoing.len(), 1);
  assert_eq!(world.get_computer(pc1.id).unwrap().outgoing.len(), 0);

  // PC2 should have a message in its incoming queue, but not outgoing
  assert_eq!(world.get_computer(pc2.id).unwrap().ingoing.len(), 1);
  assert_eq!(world.get_computer(pc2.id).unwrap().outgoing.len(), 0);
}
