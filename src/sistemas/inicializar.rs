use crate::componentes::*;
use bevy::{prelude::*, render::camera::WindowOrigin};
use crate::sistemas::*;
use std::fs;

pub fn crear_juego(mut com: Commands, mut asset_server: Res<AssetServer>) {
    let f = fs::read_to_string("./src/mapa.txt").unwrap();

    let mut camara = Camera2dBundle::default();
    camara.projection.window_origin = WindowOrigin::BottomLeft;
    com.spawn(camara);

    for (y, linea) in f.lines().rev().enumerate() {
        for (x, char) in linea.chars().enumerate() {
            let cords = Posicion::from(x as f32, y as f32);
            match char {
                'J' => crear_jugador(cords, &mut com, &mut asset_server),
                'o' => crear_caja(cords, &mut com, &mut asset_server),
                'x' => crear_objetivo(cords, &mut com, &mut asset_server),
                '-' | '|' => crear_muro(cords, &mut com, &mut asset_server),
                ' ' => {}
                invalido => panic!("formato de mapa no válido. Caracter = '{}'", invalido),
            };
        }
    }
}

fn sprite_from(
    path: &str,
    coordenadas: Posicion,
    asset_server: &mut Res<AssetServer>,
) -> SpriteBundle {
    SpriteBundle {
        texture: asset_server.load(path),
        transform: Transform {
            translation: t_from(coordenadas),
            scale: Vec3::new(SCALE, SCALE, 1f32),
            ..default()
        },
        ..default()
    }
}

fn crear_jugador(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("jugador.png", coordenadas, asset_server),
        coordenadas,
        Terminado(false),
        Movible,
        Jugador,
    ));
}

fn crear_caja(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("caja.png", coordenadas, asset_server),
        coordenadas,
        Movible,
        Terminado(false),
        Caja,
    ));
}

fn crear_objetivo(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("objetivo.png", coordenadas, asset_server),
        coordenadas,
        Fijo,
        Objetivo,
    ));
}

fn crear_muro(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("muro.png", coordenadas, asset_server),
        coordenadas,
        Fijo,
    ));
}
