use oh_no::{pc_send_blank_msg, World};

#[test]
fn it_propagates_messages() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_blank_msg);
  let pc2 = world.add_computer(pc_send_blank_msg);

  world.connect_computers(pc1, pc2);
  world.tick_run_computers();

  // Refresh the references
  let pc1 = world.get_computer(pc1).unwrap();
  let pc2 = world.get_computer(pc2).unwrap();

  // PC1 should have a message in its outgoing queue, but not incoming
  assert_eq!(pc1.outgoing.len(), 1);
  assert_eq!(pc1.incoming.len(), 0);

  // PC2 should have a message in its outgoing queue, but not incoming
  assert_eq!(pc2.outgoing.len(), 1);
  assert_eq!(pc2.incoming.len(), 0);

  world.tick_deliver_messages(true);

  // Refresh the references
  let pc1 = world.get_computer(pc1.id).unwrap();
  let pc2 = world.get_computer(pc2.id).unwrap();

  // PC1 should have a message in its incoming queue, but not outgoing
  assert_eq!(pc1.incoming.len(), 1);
  assert_eq!(pc1.outgoing.len(), 0);

  // PC2 should have a message in its incoming queue, but not outgoing
  assert_eq!(pc2.incoming.len(), 1);
  assert_eq!(pc2.outgoing.len(), 0);
}

#[test]
fn it_propagates_messages_with_tick() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_blank_msg);
  let pc2 = world.add_computer(pc_send_blank_msg);

  world.connect_computers(pc1, pc2);

  // Run two ticks (incoming queues should be cleared)
  world.tick();
  world.tick();

  // Refresh the references
  let pc1 = world.get_computer(pc1).unwrap();
  let pc2 = world.get_computer(pc2).unwrap();

  // PC1 should have a message in its incoming queue, but not outgoing
  assert_eq!(pc1.incoming.len(), 1);
  assert_eq!(pc1.outgoing.len(), 0);

  // PC2 should have a message in its incoming queue, but not outgoing
  assert_eq!(pc2.incoming.len(), 1);
  assert_eq!(pc2.outgoing.len(), 0);
}

#[test]
fn it_routes_messages_using_edge_index() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_blank_msg);
  let pc2 = world.add_computer(pc_send_blank_msg);

  world.connect_computers(pc1, pc2);
  world.tick_run_computers();

  // Refresh the references
  let pc1 = world.get_computer(pc1).unwrap();
  let pc2 = world.get_computer(pc2).unwrap();

  let pc1_outgoing_edge = pc1.outgoing.get(0).unwrap().edge;
  let pc2_outgoing_edge = pc2.outgoing.get(0).unwrap().edge;

  world.tick_deliver_messages(true);

  // Refresh the references
  let pc1 = world.get_computer(pc1.id).unwrap();
  let pc2 = world.get_computer(pc2.id).unwrap();

  // PC1 should have a message from PC2 (same edge)
  assert_eq!(pc1.incoming.get(0).unwrap().edge, pc2_outgoing_edge);

  // PC2 should have a message from PC1 (same edge)
  assert_eq!(pc2.incoming.get(0).unwrap().edge, pc1_outgoing_edge);
}

#[test]
fn it_propagates_messages_for_multiple_connections() {
  let mut world = World::default();

  // Network Structure (node densities): 3 -> 1 -> 2
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

  // A1 gets a message from A2 and B1
  // A2 gets a message from A1 and B1
  // A3 gets a message from B1
  assert_eq!(world.get_computer(pc_a1).unwrap().incoming.len(), 2);
  assert_eq!(world.get_computer(pc_a2).unwrap().incoming.len(), 2);
  assert_eq!(world.get_computer(pc_a3).unwrap().incoming.len(), 1);

  // B1 gets a message from A1, A2, A3, C1, and C2
  assert_eq!(world.get_computer(pc_b1).unwrap().incoming.len(), 5);

  // C1 gets a message from B1
  // C2 gets a message from B1
  assert_eq!(world.get_computer(pc_c1).unwrap().incoming.len(), 1);
  assert_eq!(world.get_computer(pc_c2).unwrap().incoming.len(), 1);
}
