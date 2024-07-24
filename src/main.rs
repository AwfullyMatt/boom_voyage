use bevy::app::{App, AppExit};
use boom_boyage::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}
