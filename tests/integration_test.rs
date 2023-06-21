use oh_no::{pc_send_blank_msg, World};

#[test]
fn it_propagates_messages() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_blank_msg);
  let pc2 = world.add_computer(pc_send_blank_msg);

  world.connect_computers(pc1, pc2);
  world.tick();

  // PC1 should have a message in its incoming queue, but not outgoing
  assert_eq!(world.get_computer(pc1).unwrap().incoming.len(), 1);
  assert_eq!(world.get_computer(pc1).unwrap().outgoing.len(), 0);

  // PC2 should have a message in its incoming queue, but not outgoing
  assert_eq!(world.get_computer(pc2).unwrap().incoming.len(), 1);
  assert_eq!(world.get_computer(pc2).unwrap().outgoing.len(), 0);
}

// 3 -> 1 -> 2
#[test]
fn it_propagates_messages_for_multiple_connections() {
  let mut world = World::default();
  let pc_a1 = world.add_computer(pc_send_blank_msg); // 0
  let pc_a2 = world.add_computer(pc_send_blank_msg); // 1
  let pc_a3 = world.add_computer(pc_send_blank_msg); // 2
  let pc_b1 = world.add_computer(pc_send_blank_msg); // 3
  let pc_c1 = world.add_computer(pc_send_blank_msg); // 4
  let pc_c2 = world.add_computer(pc_send_blank_msg); // 5

  world.connect_computers(pc_a1, pc_a2);

  world.connect_computers(pc_a1, pc_b1);
  world.connect_computers(pc_a2, pc_b1);
  world.connect_computers(pc_a3, pc_b1);

  world.connect_computers(pc_b1, pc_c1);
  world.connect_computers(pc_b1, pc_c2);

  world.tick();
}
