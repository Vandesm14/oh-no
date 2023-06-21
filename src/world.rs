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

pub struct AddResult {
  pub node: NodeIndex,
  pub id: ComputerID,
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
  /// Run a tick on the world
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

  /// Adds a computer to the world
  pub fn add_computer(&mut self, run: ComputerRun) -> AddResult {
    let computer = Computer::new(self.computers.len(), run);
    let computer_id = computer.id;

    let node_index = self.graph.add_node(computer_id);
    self.addressbook.insert(self.computers.len(), node_index);
    self.computers.push(RefCell::new(computer));

    AddResult {
      node: node_index,
      id: computer_id,
    }
  }

  /// Connects two computers by their node indecies
  pub fn connect_computers(&mut self, id1: NodeIndex, id2: NodeIndex) {
    self.graph.add_edge(id1, id2, ());
  }

  /// Returns a list of edges for a given computer ID
  pub fn edge_ids(&self, computer_id: ComputerID) -> Option<Vec<EdgeIndex>> {
    let node_index = self.addressbook.get_by_left(&computer_id);

    if let Some(node_index) = node_index {
      return Some(self.graph.edges(*node_index).map(|e| e.id()).collect());
    }

    None
  }

  /// Returns a clone of a computer by its ID
  pub fn get_computer(
    &self,
    computer_id: ComputerID,
  ) -> Result<Computer, &'static str> {
    let computer = self.computers.get(computer_id).expect("Computer not found");

    // FIXME: Cloning doesn't seem like the best idea, but I couldn't figure out
    // how to get a ref to the computer outside of the RefCell.
    let cloned = (computer.borrow()).clone();

    Ok(cloned)
  }

  /// Converts a NodeIndex into a ComputerID
  pub fn node_index_to_computer_id(&self, node_index: NodeIndex) -> ComputerID {
    *self.addressbook.get_by_right(&node_index).unwrap()
  }
}
