use oh_no::{pc_send_bgp_msg, World};

#[test]
fn it_propagates_messages() {
  let mut world = World::default();
  let pc1_node = world.add_computer(pc_send_bgp_msg);
  let pc2_node = world.add_computer(pc_send_bgp_msg);

  world.connect_computers(pc1_node, pc2_node);
  world.tick();

  // PC1 should have a message from PC2
  assert_eq!(
    world
      .get_computer(world.node_index_to_computer_id(pc1_node))
      .unwrap()
      .ingoing
      .len(),
    1
  );

  // PC2 should have a message from PC1
  assert_eq!(
    world
      .get_computer(world.node_index_to_computer_id(pc2_node))
      .unwrap()
      .ingoing
      .len(),
    1
  );
}
