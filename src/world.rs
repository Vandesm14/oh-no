#![forbid(unsafe_code)]

use petgraph::prelude::*;
use rune::{runtime::VmError, Vm};

use crate::External;

pub type ComputerId = u32;

#[derive(Debug, Default)]
pub struct World {
  network: UnGraph<Computer, (), ComputerId>,
}

impl World {
  #[inline]
  pub fn add_computer(&mut self, computer: Computer) -> ComputerId {
    self.network.add_node(computer).index() as ComputerId
  }

  #[inline]
  pub fn remove_computer(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<Computer> {
    self
      .network
      .remove_node(NodeIndex::new(computer_id as usize))
  }

  #[inline]
  pub fn computer(&self, computer_id: ComputerId) -> Option<&Computer> {
    self
      .network
      .node_weight(NodeIndex::new(computer_id as usize))
  }

  #[inline]
  pub fn computer_mut(
    &mut self,
    computer_id: ComputerId,
  ) -> Option<&mut Computer> {
    self
      .network
      .node_weight_mut(NodeIndex::new(computer_id as usize))
  }

  #[inline]
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

  #[inline]
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

  #[inline]
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
    for computer in self.network.node_weights_mut() {
      let result = computer.update();
      println!("{}: {:#?}", computer.id, result);
    }
  }
}

#[derive(Debug)]
pub struct Computer {
  // pub events: Vec<Event>,
  pub vm: Vm,

  /// The computer's unique identifier.
  pub id: ComputerId,
}

impl Computer {
  pub fn update(&mut self) -> Result<External, VmError> {
    rune::from_value::<External>(self.vm.call(
      ["main"],
      (External {
        id: self.id,
        outgoing: vec![],
      },),
    )?)
  }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Event {
//   Message(ComputerId),
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn a_b_c() {
//     let mut world = World::default();

//     let computer_a = world.add_computer(Computer::default());
//     let computer_b = world.add_computer(Computer::default());
//     let computer_c = world.add_computer(Computer::default());

//     world.connect(computer_a, computer_b);
//     world.connect(computer_b, computer_c);

//     assert!(world.is_connected(computer_a, computer_b));
//     assert!(!world.is_connected(computer_a, computer_c));
//     assert!(world.is_connected(computer_b, computer_a));
//     assert!(world.is_connected(computer_b, computer_c));
//     assert!(!world.is_connected(computer_c, computer_a));
//     assert!(world.is_connected(computer_c, computer_b));
//   }
// }
