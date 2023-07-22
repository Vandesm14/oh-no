use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
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
    .configure_set(Update, ComputerSet::Propagation)
    .add_systems(
      Update,
      propagate_messages_system.in_set(ComputerSet::Propagation),
    )
    .add_systems(PostUpdate, log_system)
    .add_plugins(CountOnMessage);

  app.update();
}

#[allow(clippy::type_complexity)]
fn log_system(
  query: Query<(
    Entity,
    &ComputerId,
    Option<&Counter>,
    &IncomingQueue,
    &OutgoingQueue,
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
