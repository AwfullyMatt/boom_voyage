use bevy::{prelude::*, reflect::List, window::WindowResized};

use crate::AppState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Columns::init())
            .insert_resource(Runway::init())
            .add_event::<EventSpawnAttack>()
            .add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Menu), (setup_columns, setup_runway))
            .add_systems(
                Update,
                (adjust_columns, adjust_runway, evr_spawn_attack)
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnEnter(AppState::Playing), evw_spawn_attack);
    }
}

#[derive(Resource, Clone, Copy, Default)]
pub struct Columns {
    first: Column,
    second: Column,
    third: Column,
    fourth: Column,
    fifth: Column,
}
impl Columns {
    fn init() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Column {
    u: usize,
    x: f32,
    active: bool,
}

#[derive(Resource, Clone, Copy, Default)]
pub struct Runway {
    start: f32,
    end: f32,
}
impl Runway {
    fn init() -> Self {
        Self::default()
    }
}

#[derive(Component, Clone)]
pub struct Attack {
    source: Option<Entity>,
    target: Option<Entity>,
    columns: Vec<usize>,
}

#[derive(Event, Clone, Reflect)]
pub struct EventSpawnAttack(pub Vec<usize>);

fn dummy_system() {}

fn setup_columns(query_window: Query<&Window>, mut columns: ResMut<Columns>) {
    if let Ok(window) = query_window.get_single() {
        let width = window.resolution.width();
        let column_width = width / 5.;

        columns.third.x = 0.;
        columns.first.x = columns.third.x - (column_width * 2.);
        columns.second.x = columns.third.x - column_width;
        columns.fourth.x = columns.third.x + column_width;
        columns.fifth.x = columns.fourth.x + (column_width * 2.);

        info!(
            "[INITIALIZED] Columns:\n
                First: {}\n
                Second: {}\n
                Third: {}\n
                Fourth: {}\n
                Fifth: {}\n",
            columns.first.x, columns.second.x, columns.third.x, columns.fourth.x, columns.fifth.x,
        );
    }
}

fn adjust_columns(mut ev_window_resize: EventReader<WindowResized>, mut columns: ResMut<Columns>) {
    for ev in ev_window_resize.read() {
        let width = ev.width;
        let column_width = width / 5.;
        columns.first.x = column_width / 2.;
        columns.second.x = columns.first.x + column_width;
        columns.third.x = columns.second.x + column_width;
        columns.fourth.x = columns.third.x + column_width;
        columns.fifth.x = columns.fourth.x + column_width;

        info!(
            "[MODIFIED] Columns:\n
                First: {}\n
                Second: {}\n
                Third: {}\n
                Fourth: {}\n
                Fifth: {}\n",
            columns.first.x, columns.second.x, columns.third.x, columns.fourth.x, columns.fifth.x,
        );
    }
}

fn setup_runway(query_window: Query<&Window>, mut runway: ResMut<Runway>) {
    if let Ok(window) = query_window.get_single() {
        let height = window.resolution.height();
        let start = 100.;
        let end = height - 100.;

        runway.start = start;
        runway.end = end;

        info!(
            "[INITIALIZED] Runway:\n
                Start: {}\n
                End: {}\n",
            runway.start, runway.end,
        );
    }
}

fn adjust_runway(mut ev_window_resized: EventReader<WindowResized>, mut runway: ResMut<Runway>) {
    for ev in ev_window_resized.read() {
        let height = ev.height;
        let start = 100.;
        let end = height - 100.;

        runway.start = start;
        runway.end = end;

        info!(
            "[MODIFIED] Runway:\n
                Start: {}\n
                End: {}\n",
            runway.start, runway.end,
        );
    }
}

fn evw_spawn_attack(mut evw_spawn_attack: EventWriter<EventSpawnAttack>) {
    evw_spawn_attack.send(EventSpawnAttack(vec![1, 2, 3, 4]));
}

fn evr_spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    columns: Res<Columns>,
    mut ev_spawn_attack: EventReader<EventSpawnAttack>,
) {
    for ev in ev_spawn_attack.read() {
        info!("[EVENT-READ] Spawn Attack");
        let attack = commands
            .spawn(Attack {
                source: None,
                target: None,
                columns: ev.0.clone(),
            })
            .insert(SpatialBundle {
                visibility: Visibility::Visible,
                ..default()
            })
            .id();

        for column in &ev.0 {
            let sprite_bundle = init_column_sprite(*column, &asset_server, &columns);
            let entity = commands.spawn(sprite_bundle).id();
            commands.entity(attack).push_children(&[entity]);
        }

        info!("[SPAWNED] Attack");
    }
}

fn init_column_sprite(
    u: usize,
    asset_server: &Res<AssetServer>,
    columns: &Res<Columns>,
) -> SpriteBundle {
    let texture = asset_server.load("sprites/box.png");
    let x = match u {
        1 => columns.first.x,
        2 => columns.second.x,
        3 => columns.third.x,
        4 => columns.fourth.x,
        5 => columns.fifth.x,
        _ => {
            info!("[ERROR] Unexpected Column ID -- {}", u);
            0.
        }
    };

    let sprite_bundle = SpriteBundle {
        transform: Transform::from_translation(Vec3::new(x, 0., 0.)),
        texture,
        ..default()
    };

    sprite_bundle
}
