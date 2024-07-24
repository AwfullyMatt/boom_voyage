use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system);
    }
}

#[derive(Component, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Relation {
    Player,
    Enemy,
    Ally,
}

#[derive(Component, Clone, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct CharacterName(pub String);
impl CharacterName {
    pub fn from(s: &str) -> CharacterName {
        CharacterName(s.to_string())
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct Freedom(pub i32); // health

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct Gumption(pub i32); // dexterity

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct Backbone(pub i32); // strength

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct Nephew(pub i32); // magic

fn dummy_system() {}
