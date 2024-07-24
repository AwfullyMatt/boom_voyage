use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system);
    }
}

#[derive(Component)]
pub enum Relation {
    Player,
    Enemy,
    Ally,
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Freedom(pub i32); // health

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Gumption(pub i32); // dexterity

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Backbone(pub i32); // strength

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Nephew(pub i32); // magic

fn dummy_system() {}
