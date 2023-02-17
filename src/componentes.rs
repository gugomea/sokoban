use std::ops::{Add, Mul};

use bevy::ecs::component::Component;

#[derive(Component, Default, Clone, Copy, PartialEq, Debug)]
pub struct Posicion {
    pub x: f32,
    pub y: f32,
}

impl Posicion {
    pub fn from(x: f32, y: f32) -> Posicion {
        Posicion { x, y }
    }
}

impl Add for Posicion {
    type Output = Posicion;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Posicion {
    type Output = Posicion;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Component, Debug)]
pub struct Terminado(pub bool);

// marker components: componenetes para ayudar a identificar entidades.
#[derive(Component)]
pub struct Fijo;

#[derive(Component)]
pub struct Movible;

#[derive(Component, Debug)]
pub struct Objetivo;

#[derive(Component, Debug)]
pub struct Caja;

#[derive(Component, Debug)]
pub struct Jugador;
