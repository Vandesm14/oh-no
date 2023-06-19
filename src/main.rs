use petgraph::prelude::*;

type ComputerID = usize;
type ComputerRun = fn(&Computer, &World);
type MessagePort = u8;

struct Computer {
  id: ComputerID,
  run: ComputerRun,
  ingoing: Vec<Message>,
  outgoing: Vec<Message>,
}

impl Computer {
  fn new(id: ComputerID, run: ComputerRun) -> Self {
    Computer {
      id,
      run,
      ingoing: vec![],
      outgoing: vec![],
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
  graph: UnGraph<Computer, ()>,
  max_id: ComputerID,
}

impl World {
  fn tick(&self) {
    self.graph.node_indices().for_each(|i| {
      let node = &self.graph[i];

      (node.run)(node, self);
    })
  }

  fn add_computer(&mut self, run: ComputerRun) -> NodeIndex {
    let node_index = self.graph.add_node(Computer::new(self.max_id, run));
    self.max_id += 1;

    node_index
  }

  fn connect_computers(&mut self, id1: NodeIndex, id2: NodeIndex) {
    self.graph.add_edge(id1, id2, ());
  }
}

fn pc_print_id(me: &Computer, world: &World) {
  println!("{}", me.id);
}

fn main() {
  let mut world = World::default();
  let pc1 = world.add_computer(pc_print_id);
  let pc2 = world.add_computer(pc_print_id);

  world.connect_computers(pc1, pc2);

  world.tick();
}
