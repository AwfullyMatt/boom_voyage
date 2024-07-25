use bevy::prelude::*;

use crate::AppState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Columns::init())
            .add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Loading), setup_columns)
            .add_systems(Update, adjust_columns.run_if(in_state(AppState::Playing)));
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
    x: f32,
    active: bool,
}

fn dummy_system() {}

fn setup_columns(query_window: Query<&Window>, mut columns: ResMut<Columns>) {
    if let Ok(window) = query_window.get_single() {
        let width = window.resolution.width();
        let column_width = width / 5.;
        columns.first.x = column_width / 2.;
        columns.second.x = columns.first.x + column_width;
        columns.third.x = columns.second.x + column_width;
        columns.fourth.x = columns.third.x + column_width;
        columns.fifth.x = columns.fourth.x + column_width;

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

fn adjust_columns(query_window: Query<&Window, Changed<Window>>, mut columns: ResMut<Columns>) {
    if let Ok(window) = query_window.get_single() {
        let width = window.resolution.width();
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
