use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::character::{Backbone, CharacterName, Freedom, Gumption, Nephew, Relation};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Bundle, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerBundle {
    name: CharacterName,
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
            name: CharacterName::from("Player"),
            player: Player,
            relation: Relation::Player,
            freedom: Freedom(10),
            gumption: Gumption(10),
            backbone: Backbone(10),
            nephew: Nephew(10),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Player;

fn spawn_player(mut commmands: Commands) {
    match deserialize_player() {
        Ok(player) => {
            commmands.spawn(player);
            info!("[SPAWNED] Saved Player Bundle.");
        }
        Err(e) => {
            commmands.spawn(PlayerBundle::default());
            info!("[SPAWNED] Default Player Bundle.");
            eprintln!("{:?}", e)
        }
    }
}

fn deserialize_player() -> Result<PlayerBundle, Box<dyn std::error::Error>> {
    let cmd = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(format!("{}/assets/ron/player.ron", cmd));

    info!(
        "Attempting to deserialize Player from\n
        {:?}",
        path
    );
    let file = File::open(path.clone())?;
    let reader = BufReader::new(file);
    let player_bundle: PlayerBundle = from_reader(reader)?;
    info!(
        "Player Bundle -- \n
        {:?}",
        player_bundle.clone()
    );
    Ok(player_bundle)
}
