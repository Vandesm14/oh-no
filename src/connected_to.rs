//!

#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use bevy::{
  prelude::*,
  utils::{HashMap, HashSet},
};

///
#[derive(Default)]
pub struct ConnectedToPlugin;

impl Plugin for ConnectedToPlugin {
  fn build(&self, app: &mut App) {
    app //
      .register_type::<ConnectedTo>();

    app //
      .configure_set(PostUpdate, ConnectedToSet::RemoveDisjoint);

    app //
      .add_systems(
        PostUpdate,
        remove_disjoint.in_set(ConnectedToSet::RemoveDisjoint),
      );
  }
}

///
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet,
)]
pub enum ConnectedToSet {
  /// Removes any disjoint [`Relation`]s.
  RemoveDisjoint,
}

/// A connection between and entity and one or more entities.
#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  Default,
  Deref,
  DerefMut,
  Reflect,
  FromReflect,
  Component,
)]
#[reflect(Debug, PartialEq, Default, Component)]
pub struct ConnectedTo(pub HashSet<Entity>);

fn remove_disjoint(
  mut to_remove_from: Local<HashMap<Entity, Vec<Entity>>>,
  mut query: Query<(Entity, &mut ConnectedTo)>,
) {
  to_remove_from.clear();

  for (entity, connected_to) in query.iter() {
    let mut to_remove = Vec::new();

    for (other_entity, other) in connected_to
      .iter()
      .map(|&entity| (entity, query.get(entity)))
    {
      match other {
        Ok((_, other_connected_to)) => {
          if !other_connected_to.contains(&entity) {
            to_remove.push(other_entity)
          }
        }
        Err(_) => to_remove.push(other_entity),
      }
    }

    to_remove_from.insert(entity, to_remove);
  }

  for (entity, to_remove) in to_remove_from.iter() {
    if let Ok((_, mut connected_to)) = query.get_mut(*entity) {
      for other_entity in to_remove.iter() {
        connected_to.remove(other_entity);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn remove_disjoint() {
    let mut app = App::new();

    app.add_plugins(ConnectedToPlugin);

    let e1 = app.world.spawn(ConnectedTo::default()).id();
    let e2 = app.world.spawn(ConnectedTo::default()).id();
    let e3 = app.world.spawn(ConnectedTo::default()).id();

    app
      .world
      .get_mut::<ConnectedTo>(e1)
      .unwrap()
      .extend([e1, e2]);
    app
      .world
      .get_mut::<ConnectedTo>(e2)
      .unwrap()
      .extend([e1, e3]);

    app.update();

    assert!(app.world.get::<ConnectedTo>(e1).unwrap().contains(&e1));
    assert!(app.world.get::<ConnectedTo>(e1).unwrap().contains(&e2));
    assert!(!app.world.get::<ConnectedTo>(e1).unwrap().contains(&e3));

    assert!(app.world.get::<ConnectedTo>(e2).unwrap().contains(&e1));
    assert!(!app.world.get::<ConnectedTo>(e2).unwrap().contains(&e2));
    assert!(!app.world.get::<ConnectedTo>(e2).unwrap().contains(&e3));

    assert!(!app.world.get::<ConnectedTo>(e3).unwrap().contains(&e1));
    assert!(!app.world.get::<ConnectedTo>(e3).unwrap().contains(&e2));
    assert!(!app.world.get::<ConnectedTo>(e3).unwrap().contains(&e3));
  }
}
