use std::error::Error;

use oh_no::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::default();

  let computer_a = world.add_computer(Box::new(MyPC::new(0)));
  let computer_b = world.add_computer(Box::new(MyPC::new(1)));
  let computer_c = world.add_computer(Box::new(MyPC::new(2)));

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();
  world.update();

  Ok(())
}

#[derive(Debug, Clone)]
struct MyPC {
  id: ComputerId,
  incoming: Messages,
}

impl MyPC {
  fn new(id: ComputerId) -> Self {
    Self {
      id,
      incoming: Vec::new(),
    }
  }
}

impl Computer for MyPC {
  fn id(&self) -> ComputerId {
    self.id
  }

  fn setup(&mut self) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn update(&mut self) -> Result<Vec<Message>, Box<dyn Error>> {
    println!("{}: {:?}", self.id, self.incoming);

    Ok(vec![Message {
      data: vec![],
      from: self.id,
      to: (self.id + 1) % 3,
    }])
  }

  fn incoming(&self) -> &Messages {
    &self.incoming
  }

  fn set_incoming(&mut self, messages: Messages) {
    self.incoming = messages;
  }
}
