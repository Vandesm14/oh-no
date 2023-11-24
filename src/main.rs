use std::error::Error;

use crossbeam_channel::{Receiver, Sender};
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
  sender: Sender<Message>,
  receiver: Receiver<Message>,
}

impl MyPC {
  fn new(id: ComputerId) -> Self {
    let (sender, receiver) = crossbeam_channel::unbounded();

    Self {
      id,
      sender,
      receiver,
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
    let incoming = self.receiver.try_iter().collect::<Vec<_>>();
    println!("{}: {:?}", self.id, incoming);

    Ok(vec![Message {
      data: vec![],
      from: self.id,
      to: (self.id + 1) % 3,
    }])
  }

  fn incoming(&self) -> &Sender<Message> {
    &self.sender
  }
}
