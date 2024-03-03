mod game;

use bevy::prelude::*;
use crate::game::plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
