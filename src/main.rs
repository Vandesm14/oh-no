use oh_no::{pc_send_bgp_msg, World};

fn main() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_send_bgp_msg);
  let pc2 = world.add_computer(pc_send_bgp_msg);

  world.connect_computers(pc1.node, pc2.node);
  world.tick();

  println!("{:#?}", world);
  println!("{}", world)
}
