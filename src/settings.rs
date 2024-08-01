use bevy::prelude::*;

use crate::AppState;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Settings), spawn_settings)
            .add_systems(
                Update,
                settings_button_press.run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), despawn_settings);
    }
}

fn dummy_system() {}

#[derive(Component)]
pub struct CleanupSettingsMenu;

#[derive(Component)]
pub enum SettingsMenuButton {
    Default,
    Dinky,
    Decent,
    Doggone,
}

fn spawn_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_bundle = ButtonBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(50.),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        border_radius: BorderRadius::MAX,
        background_color: Color::WHITE.into(),
        ..default()
    };

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 22.,
        color: Color::BLACK,
    };

    let settings_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 60.,
        color: Color::WHITE,
    };

    let node_bundle = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(33.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let setting_one_text = commands
        .spawn(TextBundle::from_section(
            "Winduh  ",
            settings_text_style.clone(),
        ))
        .id();

    commands
        .entity(node_bundle)
        .insert(CleanupSettingsMenu)
        .push_children(&[setting_one_text]);

    let button_one = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Dinky)
        .id();
    let button_one_text = commands
        .spawn(TextBundle::from_section(
            "Dinky \n(480p)",
            text_style.clone(),
        ))
        .id();
    let button_two = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Decent)
        .id();
    let button_two_text = commands
        .spawn(TextBundle::from_section(
            "Decent\n(1080p)",
            text_style.clone(),
        ))
        .id();
    let button_three = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Doggone)
        .id();
    let button_three_text = commands
        .spawn(TextBundle::from_section(
            "Doggone!\n(2160p)",
            text_style.clone(),
        ))
        .id();

    commands
        .entity(button_one)
        .push_children(&[button_one_text]);
    commands
        .entity(button_two)
        .push_children(&[button_two_text]);
    commands
        .entity(button_three)
        .push_children(&[button_three_text]);

    commands
        .entity(node_bundle)
        .push_children(&[button_one, button_two, button_three]);

    info!("[SPAWNED] Settings Menu Entities");
}

fn despawn_settings(
    mut commands: Commands,
    mut query_main_menu: Query<Entity, With<CleanupSettingsMenu>>,
) {
    for entity in query_main_menu.iter_mut() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Settings Menu Entities.");
    }
}

fn settings_button_press(
    mut query_button_interaction: Query<
        (&Interaction, &SettingsMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut query_window: Query<&mut Window>,
) {
    for (interaction, mmb) in &mut query_button_interaction {
        match interaction {
            Interaction::Pressed => match mmb {
                SettingsMenuButton::Dinky => {
                    let mut window = query_window.single_mut();

                    window.resolution.set(640., 480.);
                    info!(
                        "[MODIFIED] Window.Resolution >> {}x{}",
                        window.resolution.width(),
                        window.resolution.height()
                    );
                }
                SettingsMenuButton::Decent => {
                    let mut window = query_window.single_mut();
                    window.resolution.set(1920., 1080.);
                    info!(
                        "[MODIFIED] Window.Resolution >> {}x{}",
                        window.resolution.width(),
                        window.resolution.height()
                    );
                }
                SettingsMenuButton::Doggone => {
                    let mut window = query_window.single_mut();
                    window.resolution.set(3840., 2160.);
                    info!(
                        "[MODIFIED] Window.Resolution >> {}x{}",
                        window.resolution.width(),
                        window.resolution.height()
                    );
                }

                _ => {}
            },
            _ => {}
        }
    }
}
