use bimap::BiMap;
use petgraph::prelude::*;
use std::{cell::RefCell, fmt};

use crate::{Computer, ComputerID, ComputerRun, MessageQueue};

#[derive(Default, Debug)]
pub struct World {
  graph: UnGraph<ComputerID, ()>,
  addressbook: BiMap<ComputerID, NodeIndex>,
  computers: Vec<RefCell<Computer>>,
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.graph.node_indices().try_for_each(|i| {
      let computer_id = self.addressbook.get_by_right(&i).unwrap();
      let computer = self.computers.get(*computer_id).unwrap();

      writeln!(
        f,
        "{}: {} connections | messages ({} in, {} out)",
        computer.borrow().id,
        self.graph.neighbors(i).count(),
        computer.borrow().ingoing.len(),
        computer.borrow().outgoing.len()
      )
    })
  }
}

impl World {
  pub fn tick(&mut self) {
    // Run each computer
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = self.addressbook.get_by_right(&node_index).unwrap();
      let edges = self.edge_ids(*computer_id).unwrap();
      let computer = self.computers.get_mut(*computer_id).unwrap();

      let changes = (computer.borrow().run)(&computer.borrow(), edges);
      computer.borrow_mut().outgoing = changes;
    });

    // What I want it to be...
    // self.computers.iter_mut().for_each(|computer| {
    //   let edges = self.edge_ids(computer.id).unwrap();

    //   (computer.run)(computer, edges);
    // });

    // Deliver messages
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = self.addressbook.get_by_right(&node_index).unwrap();
      let computer = self.computers.get(*computer_id).unwrap();

      // Run through all outgoing messages
      computer.borrow_mut().outgoing.iter_mut().for_each(
        |(edge_index, message)| {
          let neighbor_index = self
            .graph
            .neighbors(node_index)
            .find(|n| n.index() != node_index.index())
            .unwrap();
          let neighbor_id =
            self.addressbook.get_by_right(&neighbor_index).unwrap();
          let neighbor = self.computers.get(*neighbor_id).unwrap();

          neighbor
            .borrow_mut()
            .ingoing
            .insert(*edge_index, (*message).clone());
        },
      );

      computer.borrow_mut().outgoing = MessageQueue::new();
    });
  }

  pub fn add_computer(&mut self, run: ComputerRun) -> NodeIndex {
    let computer = Computer::new(self.computers.len(), run);
    let node_index = self.graph.add_node(computer.id);
    self.addressbook.insert(self.computers.len(), node_index);
    self.computers.push(RefCell::new(computer));

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
