use std::{error::Error, fs};

use crossbeam_channel::Sender;
use petgraph::{
  dot::{Config, Dot},
  prelude::*,
};

pub type ComputerId = NodeIndex;

#[derive(Debug, Default)]
pub struct World {
  network: UnGraph<Box<dyn Computer>, ()>,
}

impl World {
  pub fn add_computer(
    &mut self,
    mut computer: Box<dyn Computer>,
  ) -> Result<ComputerId, Box<dyn Error>> {
    computer.setup()?;
    Ok(self.network.add_node(computer))
  }

  pub fn remove_computer(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<Box<dyn Computer>> {
    self.network.remove_node(computer_id)
  }

  #[allow(clippy::borrowed_box)]
  pub fn computer(
    &self,
    computer_id: ComputerId,
  ) -> Option<&Box<dyn Computer>> {
    self.network.node_weight(computer_id)
  }

  pub fn computer_mut(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<&mut Box<dyn Computer>> {
    self.network.node_weight_mut(computer_id)
  }

  pub fn connect(
    &mut self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) {
    self.network.add_edge(computer_id_a, computer_id_b, ());
  }

  pub fn disconnect(
    &mut self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) {
    if let Some(edge) = self.network.find_edge(computer_id_a, computer_id_b) {
      self.network.remove_edge(edge);
    }
  }

  pub fn is_connected(
    &self,
    computer_id_a: ComputerId,
    computer_id_b: ComputerId,
  ) -> bool {
    self.network.contains_edge(computer_id_a, computer_id_b)
  }

  pub fn update(&mut self) {
    let messages: Messages = self
      .network
      .node_indices()
      .collect::<Vec<_>>()
      // Collecting into a vec because above doesn't allow par_iter_mut
      .into_iter()
      .flat_map(|index| {
        let edges = self
          .network
          .edges_directed(index, Direction::Outgoing)
          .map(|edge| edge.id())
          .collect::<Vec<_>>();
        let computer = self.network.node_weight_mut(index).unwrap();

        let outgoing = computer.update(edges, index);
        if let Ok(outgoing) = outgoing {
          outgoing
        } else {
          vec![]
        }
      })
      .collect();

    messages.into_iter().for_each(|message| {
      let id = message.from;
      if let Some(recipients) = self.network.edge_endpoints(message.edge) {
        let recipient = if recipients.0 == id {
          recipients.1
        } else {
          recipients.0
        };

        if let Some(computer) = self.network.node_weight_mut(recipient) {
          let channel = computer.incoming();
          channel.send(message).ok();
        }
      }
    });

    fs::write(
      "graph.dot",
      format!(
        "{:?}",
        Dot::with_config(
          &self.network,
          &[Config::EdgeIndexLabel, Config::NodeIndexLabel]
        )
      ),
    )
    .expect("Unable to write file");
  }
}

#[derive(Debug, Clone)]
pub struct Message {
  pub from: NodeIndex,
  pub edge: EdgeIndex,
  pub data: Vec<u8>,
}

pub type Messages = Vec<Message>;

pub trait Computer: std::fmt::Debug + Send + Sync {
  /// The setup function runs as soon as a Computer is added to the World.
  fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;

  /// The update function of a computer. This runs every world update.
  fn update(
    &mut self,
    edges: Vec<EdgeIndex>,
    id: ComputerId,
  ) -> Result<Vec<Message>, Box<dyn std::error::Error>>;

  /// Gets a reference to the incoming message sender.
  fn incoming(&self) -> &Sender<Message>;
}
