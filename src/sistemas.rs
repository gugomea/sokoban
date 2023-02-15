use crate::componentes::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*, render::camera::WindowOrigin};
use std::fs;

const SCALE: f32 = 1.5f32;
const TAMANO: f32 = 32f32;

pub fn crear_juego(mut com: Commands, mut asset_server: Res<AssetServer>) {
    let f = fs::read_to_string("./src/mapa.txt").unwrap();

    let mut camara = Camera2dBundle::default();
    camara.projection.window_origin = WindowOrigin::BottomLeft;
    com.spawn(camara);

    for (y, linea) in f.lines().rev().enumerate() {
        for (x, char) in linea.chars().enumerate() {
            let cords = Posicion::from((x as f32, y as f32));
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

// hay que hacer un sistema con Changed<Posicion> !!

//fn mover_entidad(mut query: Query<(&mut Transform, &Posicion), Changed<Posicion>>) {
//    for (mut t, p) in query.iter_mut() {
//        t.translation = t_from(*p);
//    }
//}
//
//fn flecha_presionada(
//    // hacerla con timer para poder mantener presionada la tecla
//    teclas: Res<Input<KeyCode>>,
//    mut cajas: Query<(&mut Posicion, &Terminado)>,
//    muros: Query<&Posicion, &Fijo>,
//    mut jugador: Query<(&mut Posicion, &Terminado), With<Movible>>,
//) {
//    let dirs = vec![
//        (KeyCode::Up, (0, 1)),
//        (KeyCode::Down, (0, -1)),
//        (KeyCode::Right, (1, 0)),
//        (KeyCode::Left, (-1, 0)),
//    ];
//    let dir = dirs.iter().find(|(t, _)| teclas.just_pressed(*t));
//
//    if let Some(direccion) = dir {
//        let direccion = direccion.1;
//        let (mut pos, t) = jugador.single_mut();
//        let mut cajas = cajas.iter_mut().filter(|(p, t)| p.x == pos.x || pos.y == p.y).collect::<Vec<_>>();
//    }
//}

fn t_from(coordenadas: Posicion) -> Vec3 {
    Vec3::new(
        coordenadas.x * TAMANO * SCALE,
        coordenadas.y * TAMANO * SCALE,
        0f32,
    )
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
    ));
}

fn crear_caja(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("caja.png", coordenadas, asset_server),
        coordenadas,
        Movible,
    ));
}

fn crear_objetivo(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("objetivo.png", coordenadas, asset_server),
        coordenadas,
        Terminado(false),
        Fijo,
    ));
}

fn crear_muro(coordenadas: Posicion, com: &mut Commands, asset_server: &mut Res<AssetServer>) {
    com.spawn((
        sprite_from("muro.png", coordenadas, asset_server),
        coordenadas,
        Fijo,
    ));
}
