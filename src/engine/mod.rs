use std::{collections::HashMap, ops::{Deref, DerefMut}};

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};

use crate::common::Id;

#[derive(Debug, Default)]
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityIdMap::default()).world_mut().register_component_hooks::<Id>().on_insert(entity_map_insert).on_remove(entity_map_remove);
    }
    fn name(&self) -> &str {
        "Engine"
    }
}

#[derive(Debug, Default, Resource)]
struct EntityIdMap(HashMap<Id, Entity>);
impl Deref for EntityIdMap {
    type Target = HashMap<Id, Entity>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for EntityIdMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn entity_map_insert(mut world: DeferredWorld, context: HookContext) {
    let id = world.entity(context.entity).get::<Id>().unwrap().clone();
    let result = world.resource_mut::<EntityIdMap>().insert(id, context.entity);
    assert!(result.is_none(), "Try insert already exist id");
}
fn entity_map_remove(mut world: DeferredWorld, context: HookContext) {
    let id = world.entity(context.entity).get::<Id>().unwrap().clone();
    let result = world.resource_mut::<EntityIdMap>().remove(&id);
    assert!(result.is_some(), "Try remove non-exist id");
}
