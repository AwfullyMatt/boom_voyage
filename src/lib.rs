mod character;
mod combat;
mod enemy;
mod menu;
mod player;

use bevy::{
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use character::CharacterPlugin;
use combat::CombatPlugin;
use menu::MenuPlugin;
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
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins((PlayerPlugin, CharacterPlugin, CombatPlugin, MenuPlugin));
        app.insert_resource(Resolutions::default());
        app.insert_resource(Msaa::Off);
        app.insert_resource(ClearColor(Color::linear_rgb(0., 0., 0.)));
        app.init_state::<AppState>();
        app.init_state::<GameState>();
        app.add_systems(Startup, (spawn_camera, set_initial_resolution));
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum AppState {
    Loading,
    #[default]
    Menu,
    Playing,
}

#[derive(States, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Home,
    Combat,
}

#[derive(Resource)]
pub struct Resolutions {
    small: Vec2, // 480p
    large: Vec2, // 1080p
    ultra: Vec2, // 2160p
}
impl Resolutions {
    fn default() -> Self {
        Resolutions {
            small: Vec2::new(640., 480.),
            large: Vec2::new(1920., 1080.),
            ultra: Vec2::new(3840., 2160.),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}

fn set_initial_resolution(mut query_window: Query<&mut Window>) {
    if let Ok(mut window) = query_window.get_single_mut() {
        window.resolution.set(1920., 1080.);
        info!("[MODIFIED] Window Resolution : 1080p");
    }
}
