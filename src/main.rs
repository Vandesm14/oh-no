//!

#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use bevy::{prelude::*, utils::HashSet};
use oh_no::*;

fn main() {
  let mut app = App::new();

  app
    .add_plugins(ConnectedToPlugin)
    .add_systems(Startup, setup_system)
    .add_systems(
      Update,
      (propagate_messages_system, rom_count_on_incoming_system).chain(),
    )
    .add_systems(PostUpdate, log_system);

  app.update();
}

fn setup_system(mut commands: Commands) {
  let e1 = commands
    .spawn((
      ComputerId(0),
      ConnectedTo::default(),
      OutgoingQueue(vec![Message {
        recipient_id: 1,
        ..Default::default()
      }]),
    ))
    .id();
  let e2 = commands
    .spawn((
      ComputerId(1),
      ConnectedTo::default(),
      IncomingQueue::default(),
      OutgoingQueue::default(),
      Counter::default(),
    ))
    .id();

  commands.entity(e1).insert(ConnectedTo(HashSet::from([e2])));
  commands.entity(e2).insert(ConnectedTo(HashSet::from([e1])));
}

#[allow(clippy::type_complexity)]
fn log_system(
  query: Query<(
    Entity,
    &ComputerId,
    Option<&Counter>,
    Option<&IncomingQueue>,
    Option<&OutgoingQueue>,
  )>,
) {
  for entity in query.iter() {
    println!("{:?}", entity);
  }
}

fn propagate_messages_system(
  mut senders: Query<&mut OutgoingQueue>,
  mut receivers: Query<(&mut IncomingQueue, &ComputerId)>,
) {
  for mut sender in senders.iter_mut() {
    for message in sender.drain(0..) {
      let recipient_id = message.recipient_id;
      let mut entity = receivers
        .iter_mut()
        .find(|entity| entity.1 .0 == recipient_id)
        .unwrap();

      entity.0.push(message);
    }
  }
}

fn rom_count_on_incoming_system(
  mut counters: Query<(&IncomingQueue, &mut Counter)>,
) {
  for (incoming, mut counter) in counters.iter_mut() {
    counter.0 += incoming.len();
  }
}
