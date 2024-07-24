use bevy::prelude::*;

use crate::character::{Backbone, Freedom, Gumption, Nephew, Relation};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    relation: Relation,
    freedom: Freedom,
    gumption: Gumption,
    backbone: Backbone,
    nephew: Nephew,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            name: Name::new("Player"),
            player: Player,
            relation: Relation::Player,
            freedom: Freedom(10),
            gumption: Gumption(10),
            backbone: Backbone(10),
            nephew: Nephew(10),
        }
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commmands: Commands) {
    commmands.spawn(PlayerBundle::default());
    info!("Player spawned.");
}
