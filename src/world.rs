#![forbid(unsafe_code)]

use std::collections::HashMap;

use petgraph::prelude::*;

pub type ComputerId = u32;

#[derive(Debug, Default)]
pub struct World {
  network: UnGraph<Box<dyn Computer>, (), ComputerId>,

  message_queue: HashMap<ComputerId, Vec<Message>>,
}

impl World {
  pub fn add_computer(&mut self, computer: Box<dyn Computer>) -> ComputerId {
    self.network.add_node(computer).index() as ComputerId
  }

  pub fn remove_computer(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<Box<dyn Computer>> {
    self
      .network
      .remove_node(NodeIndex::new(computer_id as usize))
  }

  #[allow(clippy::borrowed_box)]
  pub fn computer(
    &self,
    computer_id: ComputerId,
  ) -> Option<&Box<dyn Computer>> {
    self
      .network
      .node_weight(NodeIndex::new(computer_id as usize))
  }

  pub fn computer_mut(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<&mut Box<dyn Computer>> {
    self
      .network
      .node_weight_mut(NodeIndex::new(computer_id as usize))
  }

  pub fn connect(
    &mut self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) {
    self.network.add_edge(
      NodeIndex::new(computer_id_a as usize),
      NodeIndex::new(computer_id_b as usize),
      (),
    );
  }

  pub fn disconnect(
    &mut self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) {
    if let Some(edge) = self.network.find_edge(
      NodeIndex::new(computer_id_a as usize),
      NodeIndex::new(computer_id_b as usize),
    ) {
      self.network.remove_edge(edge);
    }
  }

  pub fn is_connected(
    &self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) -> bool {
    self.network.contains_edge(
      NodeIndex::new(computer_id_a as usize),
      NodeIndex::new(computer_id_b as usize),
    )
  }

  pub fn update(&mut self) {
    let mut new_queue: HashMap<ComputerId, Messages> = HashMap::new();

    for computer in self.network.node_weights_mut() {
      let incoming = self
        .message_queue
        .remove(&computer.id())
        .unwrap_or_default();
      let outgoing = computer.update(incoming);

      if let Ok(outgoing) = outgoing {
        for message in outgoing {
          new_queue
            .entry(message.to)
            .or_insert_with(Vec::new)
            .push(message);
        }
      }
    }

    self.message_queue = new_queue;
  }
}

#[derive(Debug, Clone)]
pub struct Message {
  pub from: ComputerId,
  pub to: ComputerId,
  pub data: Vec<u8>,
}

type Messages = Vec<Message>;

pub trait Computer: std::fmt::Debug {
  /// The computer's unique identifier.
  fn id(&self) -> ComputerId;

  /// The setup function of a computer.
  fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;

  /// The update function of a computer.
  fn update(
    &mut self,
    incoming: Vec<Message>,
  ) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
}
