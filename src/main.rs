use bimap::BiMap;
use petgraph::prelude::*;
use std::{
  collections::{hash_map::Entry, HashMap},
  fmt,
};

type ComputerID = usize;
type ComputerRun = fn(&mut Computer, Vec<EdgeIndex>);
type MessagePort = u8;

struct Computer {
  id: ComputerID,
  run: ComputerRun,
  ingoing: Vec<Message>,
  outgoing: HashMap<EdgeIndex, Vec<Message>>,
}

impl Computer {
  fn new(id: ComputerID, run: ComputerRun) -> Self {
    Computer {
      id,
      run,
      ingoing: vec![],
      outgoing: HashMap::new(),
    }
  }

  fn queue_outgoing(&mut self, via_edge: EdgeIndex, message: Message) {
    match self.outgoing.entry(via_edge) {
      Entry::Occupied(mut entry) => {
        entry.get_mut().push(message);
      }
      Entry::Vacant(_) => {
        self.outgoing.insert(via_edge, vec![message]);
      }
    }
  }
}

struct Message {
  port: MessagePort,
  data: MessageData,
}

enum MessageData {
  BGPMessage { path: Vec<ComputerID> },
}

#[derive(Default)]
struct World {
  graph: UnGraph<ComputerID, ()>,
  addressbook: BiMap<ComputerID, NodeIndex>,
  computers: Vec<Computer>,
}

impl fmt::Debug for World {
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
  fn tick(&mut self) {
    self.graph.node_indices().for_each(|i| {
      let computer_id = self.addressbook.get_by_right(&i).unwrap();
      let edges = self.edge_ids(*computer_id).unwrap();
      let computer = self.computers.get_mut(*computer_id).unwrap();

      (computer.run)(computer, edges);
    })
  }

  fn add_computer(&mut self, run: ComputerRun) -> NodeIndex {
    let computer = Computer::new(self.computers.len(), run);
    let node_index = self.graph.add_node(computer.id);
    self.addressbook.insert(self.computers.len(), node_index);
    self.computers.push(computer);

    node_index
  }

  fn connect_computers(&mut self, id1: NodeIndex, id2: NodeIndex) {
    self.graph.add_edge(id1, id2, ());
  }

  fn edge_ids(&self, computer_id: ComputerID) -> Option<Vec<EdgeIndex>> {
    let node_index = self.addressbook.get_by_left(&computer_id);

    if let Some(node_index) = node_index {
      return Some(self.graph.edges(*node_index).map(|e| e.id()).collect());
    }

    None
  }
}

fn pc_print_id(me: &mut Computer, edges: Vec<EdgeIndex>) {
  edges.into_iter().for_each(|edge| {
    me.queue_outgoing(
      edge,
      Message {
        port: 0,
        data: MessageData::BGPMessage { path: vec![me.id] },
      },
    )
  });
}

fn main() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_print_id);
  let pc2 = world.add_computer(pc_print_id);

  world.connect_computers(pc1, pc2);
  world.tick();

  println!("{:?}", world);
}
