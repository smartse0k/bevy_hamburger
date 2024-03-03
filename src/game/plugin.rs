use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Plugin, Startup};
use crate::game::system::drop_material::drop_material;
use crate::game::system::setup::setup_game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game)
            .add_systems(FixedUpdate, drop_material);
    }
}
