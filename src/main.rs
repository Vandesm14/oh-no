//!

use bevy::{
  app::ScheduleRunnerPlugin,
  log::LogPlugin,
  prelude::*,
  utils::{HashMap, HashSet},
};
use oh_no::*;
use std::time::Duration;

fn main() {
  let mut app = App::new();

  app
    .add_plugins(ConnectedToPlugin)
    .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
      Duration::from_secs_f64(1.0 / 60.0),
    )))
    .add_plugins(LogPlugin::default())
    .add_systems(Startup, setup_system)
    .add_systems(
      Update,
      (propagate_messages_system, rom_count_on_incoming_system).chain(),
    )
    .add_systems(PostUpdate, log_system);

  app.update();
}

fn setup_system(mut commands: Commands) {
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
    .spawn((ComputerBundle::default().with_id(1), Counter::default()))
    .id();

  connect_entities(&mut commands, &[(e1, e2)])
}

fn connect_entities(commands: &mut Commands, entities: &[(Entity, Entity)]) {
  let mut connections: HashMap<Entity, Vec<Entity>> = HashMap::new();
  for (from, to) in entities {
    connections.entry(*from).or_insert_with(Vec::new).push(*to);
    connections.entry(*to).or_insert_with(Vec::new).push(*from);
  }

  for (from, to) in connections {
    commands
      .entity(from)
      .insert(ConnectedTo(HashSet::from_iter(to.into_iter())));
  }
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
  mut senders: Query<(&mut OutgoingQueue, &ConnectedTo)>,
  mut receivers: Query<(Entity, &mut IncomingQueue, &ComputerId)>,
) {
  for (mut outgoing, connected_to) in senders.iter_mut() {
    for message in outgoing.drain(0..) {
      let recipient_id = message.recipient_id;

      if let Some((entity, mut incoming, _)) = receivers
        .iter_mut()
        .find(|(_, _, computer_id)| computer_id.0 == recipient_id)
      {
        // Check if the recipient is connected to the sender
        if connected_to.contains(&entity) {
          incoming.0.push(message);
        } else {
          info!(
            "Computer {} is not connected to computer {}",
            recipient_id, message.port
          )
        }
      } else {
        info!("Computer {} does not exist", recipient_id)
      }
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
