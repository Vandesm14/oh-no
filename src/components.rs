use crate::MessageQueue;
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

/// Denotesa computer with an ID
#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut, Component)]
pub struct ComputerId(pub usize);
