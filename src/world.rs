use petgraph::prelude::*;
use std::{cell::{RefCell, Ref}, fmt, fs};

use crate::{Computer, ComputerId, MessageQueue};

#[derive(Default, Debug)]
pub struct World {
  graph: UnGraph<ComputerId, ()>,
  computers: Vec<RefCell<Box<dyn Computer>>>,
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.graph.node_indices().try_for_each(|i| {
      let computer = self.computers.get(i.index()).unwrap();

      writeln!(
        f,
        "{}: {} connections | messages ({} in, {} out)",
        computer.borrow().id(),
        self.graph.neighbors(i).count(),
        computer.borrow().incoming().len(),
        computer.borrow().outgoing().len()
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
      let mut computer = self.computers.get_mut(computer_id).unwrap().borrow_mut();

      *computer.incoming_mut() = MessageQueue::new();
    });
  }

  /// Run each computer and insert messages into the outgoing queue
  pub fn tick_run_computers(&mut self) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let edges = self.edge_ids(computer_id);
      let mut computer = self.computers.get_mut(computer_id).unwrap().borrow_mut();

      computer.run(edges)
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
      let mut computer = self.computers.get(computer_id).unwrap().borrow_mut();

      // Run through all outgoing messages
      computer.outgoing_mut().iter().for_each(|message| {
        let edge = self.graph.edge_endpoints(message.edge);

        if let Some(edge) = edge {
          let recipient_id = if edge.1.index() == computer_id {
            edge.0.index()
          } else {
            edge.1.index()
          };

          let mut recipient = self.computers.get(recipient_id).unwrap().borrow_mut();
          recipient.incoming_mut().push(message.clone());
        }

        // If not, drop the message
      });

      if clear_outgoing {
        *computer.outgoing_mut() = MessageQueue::new();
      }
    });
  }

  /// Clear all outgoing queues
  pub fn tick_outgoing_queues(&mut self) {
    self.graph.node_indices().for_each(|node_index| {
      let computer_id = node_index.index();
      let mut computer = self.computers.get(computer_id).unwrap().borrow_mut();

      *computer.outgoing_mut() = MessageQueue::new();
    });
  }

  /// Adds a computer to the world
  pub fn add_computer(&mut self, mut computer: Box<dyn Computer>) -> ComputerId {
    let computer_id = self.computers.len();
    *computer.id_mut() = computer_id;

    self.graph.add_node(computer_id);
    self.computers.push(RefCell::new(computer));
    Self::add_computer_data_dir(computer_id).unwrap();

    computer_id
  }

  fn add_computers_dir() -> Result<(), std::io::Error> {
    match fs::read_dir("./computers") {
      Err(_) => fs::create_dir("./computers"),
      Ok(_) => Ok(()),
    }
  }

  fn add_computer_data_dir(
    computer_id: ComputerId,
  ) -> Result<(), std::io::Error> {
    Self::add_computers_dir()?;

    match fs::read_dir(format!("./computers/{}", computer_id)) {
      Err(_) => fs::create_dir(format!("./computers/{}", computer_id)),
      Ok(_) => Ok(()),
    }
  }

  /// Connects two computers by their node indecies
  pub fn connect_computers(&mut self, id1: ComputerId, id2: ComputerId) {
    self
      .graph
      .add_edge(NodeIndex::new(id1), NodeIndex::new(id2), ());
  }

  /// Returns a list of edges for a given computer ID
  pub fn edge_ids(&self, computer_id: ComputerId) -> Vec<EdgeIndex> {
    let node_index = NodeIndex::new(computer_id);

    self.graph.edges(node_index).map(|e| e.id()).collect()
  }

  /// Returns a clone of a computer by its ID
  pub fn computer(
    &self,
    computer_id: ComputerId,
  ) -> Result<Ref<Box<dyn Computer>>, &'static str> {
    let computer = self.computers.get(computer_id).expect("Computer not found");

    Ok(computer.to_owned().borrow())
  }
}
