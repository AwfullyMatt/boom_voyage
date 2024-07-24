use bevy::prelude::*;

use crate::character::Relation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    relation: Relation,
}

#[derive(Component)]
pub struct Enemy;

fn dummy_system() {}
