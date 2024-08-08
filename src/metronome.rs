use bevy::{prelude::*, time::Stopwatch};

use crate::{AppState, GameState};

pub struct MetronomePlugin;
impl Plugin for MetronomePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tempo::init())
            .add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Playing), spawn_metronome)
            .add_systems(
                Update,
                (tick_metronome, update_current_beat).run_if(in_state(GameState::Combat)),
            );
    }
}

fn dummy_system() {}

#[derive(Resource)]
pub struct Tempo {
    bpm: u8,
    spb: f32,
    ts_top: u8,
    ts_bottom: u8,
}
impl Tempo {
    fn init() -> Self {
        let bpm: u8 = 60;
        let spb: f32 = bpm as f32 / 60.;
        let ts_top: u8 = 4;
        let ts_bottom: u8 = 4;
        info!(
            "[INITIALIZED] Tempo -- BPM: {} Time Siganture: {}/{}",
            bpm, ts_top, ts_bottom
        );

        Tempo {
            bpm,
            spb,
            ts_top,
            ts_bottom,
        }
    }
}

#[derive(Component, Clone, Deref, DerefMut, Default)]
pub struct Metronome {
    stopwatch: Stopwatch,
}

#[derive(Resource, Clone, Copy, Deref, DerefMut)]
pub struct CurrentBeat(pub u8);

fn spawn_metronome(mut commands: Commands) {
    commands.spawn(Metronome::default());
    info!("[SPAWNED] Metronome");
}

fn tick_metronome(mut query_metronome: Query<&mut Metronome>, time: Res<Time>) {
    if let Ok(mut metronome) = query_metronome.get_single_mut() {
        metronome.stopwatch.tick(time.delta());
    }
}

fn update_current_beat(
    mut query_metronome: Query<&mut Metronome>,
    tempo: Res<Tempo>,
    mut current_beat: ResMut<CurrentBeat>,
) {
    if let Ok(mut metronome) = query_metronome.get_single_mut() {
        if metronome.stopwatch.elapsed_secs() >= tempo.spb {
            metronome.stopwatch.reset();
            if current_beat.0 >= tempo.ts_top {
                current_beat.0 = 0;
            } else {
                current_beat.0 += 1;
            }
            info!("[MODIFIED] Metronome -- Stopwatch Reset");
            info!("[MODIFIED] Current Beat -- {}", current_beat.0);
        }
    }
}
