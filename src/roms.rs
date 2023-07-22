use bevy::prelude::*;

use crate::{
  connect_entities, ComputerBundle, Counter, IncomingQueue, Message,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ComputerSet {
  Propagation,
}

pub struct CountOnMessage;

impl Plugin for CountOnMessage {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_count_on_message)
      .add_systems(
        Update,
        rom_count_on_incoming_system.after(ComputerSet::Propagation),
      );
  }
}

fn setup_count_on_message(mut commands: Commands) {
  let e1 =
    commands
      .spawn(ComputerBundle::default().with_id(0).with_messages(vec![
        Message {
          recipient_id: 1,
          ..Default::default()
        },
      ]))
      .id();
  let e2 = commands
    .spawn((
      ComputerBundle::default()
        .with_id(1)
        .with_messages(vec![Message {
          recipient_id: 0,
          ..Default::default()
        }]),
      Counter::default(),
    ))
    .id();

  connect_entities(&mut commands, &[(e1, e2)])
}

fn rom_count_on_incoming_system(
  mut counters: Query<(&IncomingQueue, &mut Counter)>,
) {
  for (incoming, mut counter) in counters.iter_mut() {
    counter.0 += incoming.len();
  }
}
