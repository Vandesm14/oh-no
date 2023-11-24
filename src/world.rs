use petgraph::prelude::*;
use rayon::prelude::*;

pub type ComputerId = u32;

#[derive(Debug, Default)]
pub struct World {
  network: UnGraph<Box<dyn Computer>, (), ComputerId>,
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
    let messages: Messages = self
      .network
      .node_weights_mut()
      .collect::<Vec<_>>()
      .par_iter_mut()
      .flat_map(|computer| {
        let outgoing = computer.update();
        if let Ok(outgoing) = outgoing {
          outgoing
        } else {
          vec![]
        }
      })
      .collect();

    messages.into_iter().for_each(|message| {
      let computer = self.computer_mut(message.to).unwrap();
      let mut incoming = computer.incoming().clone();
      incoming.push(message);
      computer.set_incoming(incoming);
    });
  }
}

#[derive(Debug, Clone)]
pub struct Message {
  pub from: ComputerId,
  pub to: ComputerId,
  pub data: Vec<u8>,
}

pub type Messages = Vec<Message>;

pub trait Computer: std::fmt::Debug + Send {
  /// The computer's unique identifier.
  fn id(&self) -> ComputerId;

  /// The setup function of a computer.
  fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;

  /// The update function of a computer.
  fn update(&mut self) -> Result<Vec<Message>, Box<dyn std::error::Error>>;

  /// Gets incoming message queue.
  fn incoming(&self) -> &Messages;

  /// Sets incoming message queue.
  fn set_incoming(&mut self, messages: Messages);
}
