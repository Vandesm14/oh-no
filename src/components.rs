use crate::{ConnectedTo, Message, MessageQueue};
use bevy::prelude::*;

/// A queue for incoming (received) messages
#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut, Component)]
pub struct IncomingQueue(pub MessageQueue);

/// A queue for outgoing (sent) messages
#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut, Component)]
pub struct OutgoingQueue(pub MessageQueue);

/// A counter
#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut, Component)]
pub struct Counter(pub usize);

/// Denotes a computer with an ID
#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut, Component)]
pub struct ComputerId(pub usize);

/// The essential components of a computer
#[derive(Debug, Clone, PartialEq, Eq, Default, Bundle)]
pub struct ComputerBundle {
  pub id: ComputerId,
  pub incoming: IncomingQueue,
  pub outgoing: OutgoingQueue,
  pub connected_to: ConnectedTo,
}

impl ComputerBundle {
  /// Adds an id to a [`Computer`]
  pub fn with_id(mut self, id: usize) -> Self {
    self.id = ComputerId(id);
    self
  }

  /// Adds a counter to a [`Computer`]
  pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
    self.outgoing = OutgoingQueue(messages);
    self
  }
}
