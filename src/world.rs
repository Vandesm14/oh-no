use petgraph::prelude::*;
use std::{cell::RefCell, collections::HashSet, fmt};

use crate::{Computer, ComputerID, ComputerRun, MessageQueue};

#[derive(Default, Debug)]
pub struct World {
  graph: UnGraph<ComputerID, ()>,
  computers: Vec<RefCell<Computer>>,
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.graph.node_indices().try_for_each(|i| {
      let computer = self.computers.get(i.index()).unwrap();

      writeln!(
        f,
        "{}: {} connections | messages ({} in, {} out)",
        computer.borrow().id,
        self.graph.neighbors(i).count(),
        computer.borrow().incoming.len(),
        computer.borrow().outgoing.len()
      )
    })
  }
}

impl World {
  /// Run a tick on the world
  pub fn tick(&mut self) {
    self.tick_run_computers();
    self.tick_incoming_queues();
    self.tick_deliver_messages(true);
  }

  /// Clear all incoming queues
  pub fn tick_incoming_queues(&mut self) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let computer = self.computers.get(computer_id).unwrap();

      computer.borrow_mut().incoming = MessageQueue::new();
    });
  }

  /// Run each computer and insert messages into the outgoing queue
  pub fn tick_run_computers(&mut self) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let edges = self.edge_ids(computer_id);
      let computer = self.computers.get_mut(computer_id).unwrap();

      let queue = (computer.borrow().run)(&computer.borrow(), edges);
      let mut computer_mut = computer.borrow_mut();
      computer_mut.outgoing = queue;
    });

    // What I want it to be...
    // self.computers.iter_mut().for_each(|computer| {
    //   let edges = self.edge_ids(computer.borrow().id);

    //   (computer.borrow().run)(&computer.borrow(), edges);
    // });
  }

  /// Deliver messages to the correct computers
  pub fn tick_deliver_messages(&mut self, clear_outgoing: bool) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let computer = self.computers.get(computer_id).unwrap();

      // Run through all outgoing messages
      computer.borrow_mut().outgoing.iter().for_each(|message| {
        let edge = self.graph.edge_endpoints(message.edge);

        if let Some(edge) = edge {
          let recipient_id = if edge.1.index() == computer_id {
            edge.0.index()
          } else {
            edge.1.index()
          };

          let recipient = self.computers.get(recipient_id).unwrap();

          recipient.borrow_mut().incoming.push(message.clone());
        }

        // If not, drop the message
      });

      if clear_outgoing {
        computer.borrow_mut().outgoing = MessageQueue::new();
      }
    });
  }

  /// Clear all outgoing queues
  pub fn tick_outgoing_queues(&mut self) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let computer = self.computers.get(computer_id).unwrap();

      computer.borrow_mut().outgoing = MessageQueue::new();
    });
  }

  /// Adds a computer to the world
  pub fn add_computer(&mut self, run: ComputerRun) -> ComputerID {
    let computer = Computer::new(self.computers.len(), run);
    let computer_id = computer.id;

    self.graph.add_node(computer_id);
    self.computers.push(RefCell::new(computer));

    computer_id
  }

  /// Connects two computers by their node indecies
  pub fn connect_computers(&mut self, id1: ComputerID, id2: ComputerID) {
    self
      .graph
      .add_edge(NodeIndex::new(id1), NodeIndex::new(id2), ());
  }

  /// Returns a list of edges for a given computer ID
  pub fn edge_ids(&self, computer_id: ComputerID) -> Vec<EdgeIndex> {
    let node_index = NodeIndex::new(computer_id);

    self.graph.edges(node_index).map(|e| e.id()).collect()
  }

  /// Returns a clone of a computer by its ID
  pub fn get_computer(
    &self,
    computer_id: ComputerID,
  ) -> Result<Computer, &'static str> {
    let computer = self.computers.get(computer_id).expect("Computer not found");

    Ok(computer.to_owned().into_inner())
  }
}
