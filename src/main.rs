#![allow(dead_code)]

use bevy::{ecs::world::SpawnBatchIter, prelude::*};
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
        .add_system(sistemas::flecha_presionada)
        .add_system(sistemas::mover_entidad)
        .add_system(sistemas::comprobar_objetivo)
        .run();
}
