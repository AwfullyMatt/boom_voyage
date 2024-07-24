mod character;
mod enemy;
mod player;

use bevy::{
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use character::CharacterPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Boom Voyage".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.5),
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }),
        );
        app.add_plugins((PlayerPlugin, CharacterPlugin));
        app.insert_resource(Msaa::Off);
        app.insert_resource(ClearColor(Color::linear_rgb(0., 0., 0.)));
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}
