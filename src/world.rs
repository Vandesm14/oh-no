use bimap::BiMap;
use petgraph::prelude::*;
use std::fmt;

use crate::{Computer, ComputerID, ComputerRun};

#[derive(Default, Debug)]
pub struct World {
  graph: UnGraph<ComputerID, ()>,
  addressbook: BiMap<ComputerID, NodeIndex>,
  computers: Vec<Computer>,
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.graph.node_indices().try_for_each(|i| {
      let computer_id = self.addressbook.get_by_right(&i).unwrap();
      let computer = self.computers.get(*computer_id).unwrap();

      writeln!(
        f,
        "{}: {} connections | messages ({} in, {} out)",
        computer.id,
        self.graph.neighbors(i).count(),
        computer.ingoing.len(),
        computer.outgoing.len()
      )
    })
  }
}

impl World {
  pub fn tick(&mut self) {
    self.graph.node_indices().for_each(|i| {
      let computer_id = self.addressbook.get_by_right(&i).unwrap();
      let edges = self.edge_ids(*computer_id).unwrap();
      let computer = self.computers.get_mut(*computer_id).unwrap();

      (computer.run)(computer, edges);
    })
  }

  pub fn add_computer(&mut self, run: ComputerRun) -> NodeIndex {
    let computer = Computer::new(self.computers.len(), run);
    let node_index = self.graph.add_node(computer.id);
    self.addressbook.insert(self.computers.len(), node_index);
    self.computers.push(computer);

    node_index
  }

  pub fn connect_computers(&mut self, id1: NodeIndex, id2: NodeIndex) {
    self.graph.add_edge(id1, id2, ());
  }

  pub fn edge_ids(&self, computer_id: ComputerID) -> Option<Vec<EdgeIndex>> {
    let node_index = self.addressbook.get_by_left(&computer_id);

    if let Some(node_index) = node_index {
      return Some(self.graph.edges(*node_index).map(|e| e.id()).collect());
    }

    None
  }
}
