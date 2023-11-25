use std::error::Error;

use crossbeam_channel::{Receiver, Sender};
use oh_no::*;
use petgraph::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::default();

  let computer_a = world.add_computer(Box::new(MyPC::new())).unwrap();
  let computer_b = world.add_computer(Box::new(MyPC::new())).unwrap();
  let computer_c = world.add_computer(Box::new(MyPC::new())).unwrap();

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();
  world.update();

  Ok(())
}

#[derive(Debug, Clone)]
struct MyPC {
  sender: Sender<Message>,
  receiver: Receiver<Message>,
}

impl MyPC {
  fn new() -> Self {
    let (sender, receiver) = crossbeam_channel::unbounded();

    Self { sender, receiver }
  }
}

impl Computer for MyPC {
  fn setup(&mut self) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  fn update(
    &mut self,
    edges: Vec<EdgeIndex>,
    id: ComputerId,
  ) -> Result<Vec<Message>, Box<dyn Error>> {
    let incoming = self.receiver.try_iter().collect::<Vec<_>>();
    println!("{}: {:?}", id, incoming);

    // Ok(vec![Message { data: vec![] }])
    Ok(
      edges
        .into_iter()
        .map(|edge| Message {
          data: vec![],
          edge,
          from: NodeIndex::from(id),
        })
        .collect(),
    )
  }

  fn incoming(&self) -> &Sender<Message> {
    &self.sender
  }
}
