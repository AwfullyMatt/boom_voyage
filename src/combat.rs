use bevy::{prelude::*, window::WindowResized};

use crate::{AppState, GameState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Columns::init())
            .insert_resource(Runway::init())
            .init_resource::<CombatButtons>()
            .add_event::<EventSpawnAttack>()
            .add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Menu), (setup_columns, setup_runway))
            .add_systems(
                Update,
                (adjust_columns, adjust_runway, evr_spawn_attack)
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(Update, block_attack.run_if(in_state(GameState::Combat)))
            .add_systems(OnEnter(AppState::Playing), evw_spawn_attack);
    }
}

#[derive(Resource, Clone, Default)]
pub struct Columns(pub Vec<Column>);
impl Columns {
    fn init() -> Self {
        Columns(vec![
            Column {
                u: 0,
                x: 0.,
                column_active: true,
                attack_active: false,
            },
            Column {
                u: 0,
                x: 0.,
                column_active: true,
                attack_active: false,
            },
            Column {
                u: 0,
                x: 0.,
                column_active: true,
                attack_active: false,
            },
            Column {
                u: 0,
                x: 0.,
                column_active: true,
                attack_active: false,
            },
            Column {
                u: 0,
                x: 0.,
                column_active: true,
                attack_active: false,
            },
        ])
    }
}

#[derive(Clone, Copy, Default)]
pub struct Column {
    u: usize,
    x: f32,
    column_active: bool,
    attack_active: bool,
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

#[derive(Component)]
pub struct Attack {
    source: Option<Source>,
    target: Option<Target>,
    columns: Vec<usize>,
}

#[derive(Component, Clone)]
pub struct Source(pub Entity);

#[derive(Component, Clone)]
pub struct Target(pub Entity);

#[derive(Event, Clone, Reflect)]
pub struct EventSpawnAttack(pub Vec<usize>);

#[derive(Resource, Clone, Copy, Default)]
pub enum CombatButtons {
    #[default]
    LaneOne,
    LaneTwo,
    LaneThree,
    LaneFour,
    LaneFive,
}
impl CombatButtons {
    pub const COMBAT_BUTTONS: [CombatButtons; 5] = [
        CombatButtons::LaneOne,
        CombatButtons::LaneTwo,
        CombatButtons::LaneThree,
        CombatButtons::LaneFour,
        CombatButtons::LaneFive,
    ];

    fn keycode(&self) -> Option<KeyCode> {
        match self {
            CombatButtons::LaneOne => Some(KeyCode::KeyA),
            CombatButtons::LaneTwo => Some(KeyCode::KeyS),
            CombatButtons::LaneThree => Some(KeyCode::KeyD),
            CombatButtons::LaneFour => Some(KeyCode::KeyF),
            CombatButtons::LaneFive => Some(KeyCode::KeyG),
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            CombatButtons::LaneOne => Some(1),
            CombatButtons::LaneTwo => Some(2),
            CombatButtons::LaneThree => Some(3),
            CombatButtons::LaneFour => Some(4),
            CombatButtons::LaneFive => Some(5),
        }
    }
}

fn dummy_system() {}

fn setup_columns(query_window: Query<&Window>, mut columns: ResMut<Columns>) {
    if let Ok(window) = query_window.get_single() {
        let width = window.resolution.width();
        let column_width = width / 5.;

        columns.0[2].x = 0.;
        columns.0[0].x = columns.0[2].x - (column_width * 2.);
        columns.0[1].x = columns.0[2].x - column_width;
        columns.0[3].x = columns.0[2].x + column_width;
        columns.0[4].x = columns.0[3].x + (column_width * 2.);

        info!(
            "[INITIALIZED] Columns:\n
                First: {}\n
                Second: {}\n
                Third: {}\n
                Fourth: {}\n
                Fifth: {}\n",
            columns.0[0].x, columns.0[1].x, columns.0[2].x, columns.0[3].x, columns.0[4].x,
        );
    }
}

fn adjust_columns(mut ev_window_resize: EventReader<WindowResized>, mut columns: ResMut<Columns>) {
    for ev in ev_window_resize.read() {
        let width = ev.width;
        let column_width = width / 5.;

        columns.0[2].x = 0.;
        columns.0[0].x = columns.0[2].x - (column_width * 2.);
        columns.0[1].x = columns.0[2].x - column_width;
        columns.0[3].x = columns.0[2].x + column_width;
        columns.0[4].x = columns.0[3].x + (column_width * 2.);

        info!(
            "[MODIFIED] Columns:\n
                First: {}\n
                Second: {}\n
                Third: {}\n
                Fourth: {}\n
                Fifth: {}\n",
            columns.0[0].x, columns.0[1].x, columns.0[2].x, columns.0[3].x, columns.0[4].x,
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
            "[INITIALIZED] Runway -- Start: {} End: {}",
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
            "[MODIFIED] Runway -- Start: {} End: {}",
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
        1 => columns.0[0].x,
        2 => columns.0[1].x,
        3 => columns.0[2].x,
        4 => columns.0[3].x,
        5 => columns.0[4].x,
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

fn block_attack(button_input: Res<ButtonInput<KeyCode>>, columns: Res<Columns>) {
    for button in CombatButtons::COMBAT_BUTTONS {
        if button_input.just_pressed(button.keycode().unwrap()) {
            match button.id() {
                Some(n) => {
                    let column = columns.0.iter().nth(n).unwrap();
                    if column.column_active {
                        if column.attack_active {
                        } else {
                        }
                    } else {
                        info!("[IGNORING] Column Index {n} not active.");
                    }
                }
                None => {}
            }
        }
    }
}
