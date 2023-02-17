mod movimientos;
mod inicializar;
use crate::componentes::*;
use bevy::prelude::*;

pub use movimientos::*;
pub use inicializar::*;

pub const SCALE: f32 = 1.5f32;
pub const TAMANO: f32 = 32f32;

pub fn t_from(coordenadas: Posicion) -> Vec3 {
    Vec3::new(
        coordenadas.x * TAMANO * SCALE,
        coordenadas.y * TAMANO * SCALE,
        0f32,
    )
}
