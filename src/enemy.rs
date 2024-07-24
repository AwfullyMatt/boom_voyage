use bevy::prelude::*;

use crate::character::{Backbone, CharacterName, Freedom, Gumption, Nephew, Relation};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    name: CharacterName,
    enemy: Enemy,
    relation: Relation,
    freedom: Freedom,
    gumption: Gumption,
    backbone: Backbone,
    nephew: Nephew,
}

#[derive(Component)]
pub struct Enemy;

fn dummy_system() {}
