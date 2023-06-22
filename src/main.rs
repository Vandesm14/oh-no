use oh_no::{World, RomBgpComputer};

fn main() {
  let mut world = World::default();
  let pc1 = world.add_computer(Box::<RomBgpComputer>::default());
  let pc2 = world.add_computer(Box::<RomBgpComputer>::default());

  world.connect_computers(pc1, pc2);
  world.tick_run_computers();

  println!("{:#?}", world);
  println!("{}", world);
}
