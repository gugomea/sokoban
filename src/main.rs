#![allow(dead_code)]

use bevy::prelude::*;
mod componentes;
mod entidades;
mod recursos;
mod sistemas;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Sokoban".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(sistemas::crear_juego)
        .run();
}
