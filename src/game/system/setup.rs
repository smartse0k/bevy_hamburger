use bevy::log::{debug, info};
use bevy::math::Vec3;
use bevy::prelude::{Camera2dBundle, Color, Commands, default, Sprite, SpriteBundle, Transform};
use crate::game::component::material::Material;

pub fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        (
            Material,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 10.0, 0.0),
                    scale: Vec3::new(200.0, 25.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::GOLD,
                    ..default()
                },
                ..default()
            }
        ),
    );

    info!("[setup_game] setup game done");
}
