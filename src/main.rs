use std::{error::Error, fmt};

use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, Default)]
struct MyPC;

impl Computer for MyPC {
  fn update(&mut self, incoming: &[Message], outgoing: &mut Vec<Message>) {
    println!("messages: {incoming:#?}");
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::default();

  let computer_a = world.add_node(Node::new(Box::new(MyPC), Vec::new()));
  let computer_b = world.add_node(Node::new(Box::new(MyPC), Vec::new()));
  let computer_c = world.add_node(Node::new(Box::new(MyPC), Vec::new()));

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();

  println!("{world:#?}");

  Ok(())
}

#[derive(Debug)]
enum Connection {
  Wired,
  Wireless { distance: f32 },
}

#[derive(Debug)]
struct Message;

trait Interface: fmt::Debug {
  fn handle(&mut self, message: Message) -> Option<Message>;
}

trait Computer: fmt::Debug {
  fn update(&mut self, incoming: &[Message], outgoing: &mut Vec<Message>);
}

#[derive(Debug)]
struct Node {
  computer: Box<dyn Computer>,
  interfaces: Vec<usize>,
}

impl Node {
  #[inline]
  pub const fn new(
    computer: Box<dyn Computer>,
    interfaces: Vec<usize>,
  ) -> Self {
    Self {
      computer,
      interfaces,
    }
  }
}

#[derive(Debug, Default)]
struct World {
  interfaces: Vec<Box<dyn Interface>>,
  computers: UnGraph<Node, ()>,
  messages: Vec<Message>,
  next_messages: Vec<Message>,
}

impl World {
  pub fn add_node(&mut self, node: Node) -> NodeIndex {
    self.computers.add_node(node)
  }

  pub fn connect(&mut self, a: NodeIndex, b: NodeIndex) {
    self.computers.add_edge(a, b, ());
  }

  pub fn update(&mut self) {
    for node in self.computers.node_weights_mut() {
      node
        .computer
        .update(&self.messages, &mut self.next_messages);
    }

    self.messages = core::mem::take(&mut self.next_messages);
  }
}
