use oh_no::{pc_send_bgp_msg, World};

fn main() {
  let mut world = World::default();
  let pc_a1 = world.add_computer(pc_send_bgp_msg, None); // 0
  let pc_a2 = world.add_computer(pc_send_bgp_msg, None); // 1
  let pc_a3 = world.add_computer(pc_send_bgp_msg, None); // 2
  let pc_b1 = world.add_computer(pc_send_bgp_msg, None); // 3
  let pc_c1 = world.add_computer(pc_send_bgp_msg, None); // 4
  let pc_c2 = world.add_computer(pc_send_bgp_msg, None); // 5

  world.connect_computers(pc_a1, pc_a2);

  world.connect_computers(pc_a1, pc_b1);
  world.connect_computers(pc_a2, pc_b1);
  world.connect_computers(pc_a3, pc_b1);

  world.connect_computers(pc_b1, pc_c1);
  world.connect_computers(pc_b1, pc_c2);

  world.tick();

  println!("{:#?}", world);
  println!("{}", world);
}
