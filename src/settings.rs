use bevy::prelude::*;

use crate::{AppState, Resolutions};

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
    Dinky,
    Decent,
    Doggone,
    Freedom,
    Nephew,
    Vibrate,
}

fn spawn_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TEMPLATES

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

    // PARENT NODE

    let parent_node_bundle = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();

    // WINDOW NODE

    let window_node_bundle = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let window_text = commands
        .spawn(TextBundle::from_section(
            "Window  ",
            settings_text_style.clone(),
        ))
        .id();

    commands
        .entity(window_node_bundle)
        .insert(CleanupSettingsMenu)
        .push_children(&[window_text]);

    let window_button_one = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Dinky)
        .id();
    let window_button_one_text = commands
        .spawn(TextBundle::from_section(
            "Dinky \n(480p)",
            text_style.clone(),
        ))
        .id();
    let window_button_two = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Decent)
        .id();
    let window_button_two_text = commands
        .spawn(TextBundle::from_section(
            "Decent\n(1080p)",
            text_style.clone(),
        ))
        .id();
    let window_button_three = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Doggone)
        .id();
    let window_button_three_text = commands
        .spawn(TextBundle::from_section(
            "Doggone!\n(2160p)",
            text_style.clone(),
        ))
        .id();

    commands
        .entity(window_button_one)
        .push_children(&[window_button_one_text]);
    commands
        .entity(window_button_two)
        .push_children(&[window_button_two_text]);
    commands
        .entity(window_button_three)
        .push_children(&[window_button_three_text]);

    commands.entity(window_node_bundle).push_children(&[
        window_button_one,
        window_button_two,
        window_button_three,
    ]);

    // ACCESSIBILITY NODE

    let accessibility_node_bundle = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let accessibility_text = commands
        .spawn(TextBundle::from_section(
            "Accessibility  ",
            settings_text_style.clone(),
        ))
        .id(); // these will just be on/off toggles

    commands
        .entity(accessibility_node_bundle)
        .insert(CleanupSettingsMenu)
        .push_children(&[accessibility_text]);

    let accessibility_button_one = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Freedom)
        .id();
    let accessibility_button_one_text = commands
        .spawn(TextBundle::from_section(
            "Infinite\nFreedom",
            text_style.clone(),
        ))
        .id();
    let accessibility_button_two = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Nephew)
        .id();
    let accessibility_button_two_text = commands
        .spawn(TextBundle::from_section(
            "Infinite\nNephew",
            text_style.clone(),
        ))
        .id();
    let accessibility_button_three = commands
        .spawn(button_bundle.clone())
        .insert(SettingsMenuButton::Vibrate)
        .id();
    let accessibility_button_three_text = commands
        .spawn(TextBundle::from_section(
            "Rhythm\nVibrate",  // this will probably deplete
            text_style.clone(), // controller charge quickly?
        ))
        .id();

    commands
        .entity(accessibility_button_one)
        .push_children(&[accessibility_button_one_text]);
    commands
        .entity(accessibility_button_two)
        .push_children(&[accessibility_button_two_text]);
    commands
        .entity(accessibility_button_three)
        .push_children(&[accessibility_button_three_text]);

    commands.entity(accessibility_node_bundle).push_children(&[
        accessibility_button_one,
        accessibility_button_two,
        accessibility_button_three,
    ]);

    commands
        .entity(parent_node_bundle)
        .push_children(&[window_node_bundle, accessibility_node_bundle]);

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
    resolutions: Res<Resolutions>,
) {
    for (interaction, mmb) in &mut query_button_interaction {
        match interaction {
            Interaction::Pressed => match mmb {
                SettingsMenuButton::Dinky => {
                    let mut window = query_window.single_mut();

                    window
                        .resolution
                        .set(resolutions.small.x, resolutions.small.y);
                    info!(
                        "[MODIFIED] Window.Resolution >> {}x{}",
                        window.resolution.width(),
                        window.resolution.height()
                    );
                }
                SettingsMenuButton::Decent => {
                    let mut window = query_window.single_mut();
                    window
                        .resolution
                        .set(resolutions.large.x, resolutions.large.y);
                    info!(
                        "[MODIFIED] Window.Resolution >> {}x{}",
                        window.resolution.width(),
                        window.resolution.height()
                    );
                }
                SettingsMenuButton::Doggone => {
                    let mut window = query_window.single_mut();
                    window
                        .resolution
                        .set(resolutions.ultra.x, resolutions.ultra.y);
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
