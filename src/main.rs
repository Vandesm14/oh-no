use std::error::Error;

use oh_no::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::default();

  let computer_a = world.add_computer(create_computer(0)?);
  let computer_b = world.add_computer(create_computer(1)?);
  let computer_c = world.add_computer(create_computer(2)?);

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();
  world.update();

  Ok(())
}

#[derive(Debug, Clone, Copy)]
struct MyPC {
  id: ComputerId,
}

impl Computer for MyPC {
  fn id(&self) -> ComputerId {
    self.id
  }

  fn setup(&mut self) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn update(
    &mut self,
    incoming: Vec<Message>,
  ) -> Result<Vec<Message>, Box<dyn Error>> {
    println!("{}: {:?}", self.id, incoming);

    Ok(vec![Message {
      data: vec![],
      from: self.id,
      to: self.id,
    }])
  }
}

fn create_computer(
  id: ComputerId,
) -> Result<Box<dyn Computer>, Box<dyn Error>> {
  Ok(Box::new(MyPC { id }))
}
