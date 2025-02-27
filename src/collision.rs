use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{consts, metadata::ColliderMeta};

/// Empty struct simply for grouping collision layer constants.
#[derive(Copy, Clone)]
pub struct BodyLayers;

impl BodyLayers {
    // Each successive layer represents a different bit in the 32-bit u32 type.
    //
    // The layer is represented by 1 shifted 0 places to the left:          0b0001.
    // The second layer is represented by 1 shifted one place to the left:  0b0010.
    // And so on for the rest of the layers.
    pub const ENEMY: u32 = 1 << 0;
    pub const PLAYER: u32 = 1 << 1;
    pub const PLAYER_ATTACK: u32 = 1 << 2;
    pub const ENEMY_ATTACK: u32 = 1 << 3;
    pub const BREAKABLE_ITEM: u32 = 1 << 4;
    // u32::MAX is a u32 with all of it's bits set to 1, so this will contain all of the layers.
    pub const ALL: u32 = u32::MAX;
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub active_collision_types: ActiveCollisionTypes,
    pub collision_groups: CollisionGroups,
}

impl PhysicsBundle {
    pub fn new(meta: &ColliderMeta, body_layers: u32) -> Self {
        PhysicsBundle {
            collider: (Collider::cuboid(meta.size.x / 2., meta.size.y / 2.)),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::STATIC_STATIC,
            collision_groups: CollisionGroups::new(body_layers, BodyLayers::ALL),
        }
    }
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        PhysicsBundle {
            collider: (Collider::cuboid(
                consts::PLAYER_SPRITE_WIDTH / 8.,
                consts::PLAYER_HITBOX_HEIGHT / 8.,
            )),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::STATIC_STATIC,
            collision_groups: CollisionGroups::default(),
        }
    }
}
