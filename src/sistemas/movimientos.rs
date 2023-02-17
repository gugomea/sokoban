use crate::componentes::*;
use bevy::{app::AppExit, prelude::*};
use crate::sistemas::*;

pub fn mover_entidad(mut query: Query<(&mut Transform, &Posicion), Changed<Posicion>>) {
    for (mut t, p) in query.iter_mut() {
        t.translation = t_from(*p);
    }
}

pub fn comprobar_objetivo(
    mut caja: Query<(&Posicion, &mut Terminado), With<Caja>>,
    objetivos: Query<&Posicion, With<Objetivo>>,
    mut exit: EventWriter<AppExit>,
) {
    for (posicion_caja, mut termiando_caja) in caja.iter_mut() {
        let caja_termianda = objetivos.iter().find(|p| *p == posicion_caja);
        if let Some(_) = caja_termianda {
            termiando_caja.0 = true;
        }
    }
    if let None = caja.iter().find(|(_, t)| !t.0) {
        println!("Juego Terminado");
        exit.send(AppExit);
    }
}

pub fn flecha_presionada(
    // hacerla con timer para poder mantener presionada la tecla
    teclas: Res<Input<KeyCode>>,
    cajas: Query<(Entity, &Posicion, &Terminado), With<Caja>>,
    muros: Query<&Posicion, (With<Fijo>, Without<Objetivo>)>,
    jugador: Query<(Entity, &Posicion, &Terminado), With<Jugador>>,
    cualquier_cosa: Query<&Posicion, Without<Objetivo>>,
    mut com: Commands,
) {
    let (id_jugador, posicion_jugador, terminado_jugador) = jugador.single();
    if terminado_jugador.0 {
        return;
    }
    let dirs = vec![
        (KeyCode::Up, Posicion::from(0.0, 1.0)),
        (KeyCode::Down, Posicion::from(0.0, -1.0)),
        (KeyCode::Right, Posicion::from(1.0, 0.0)),
        (KeyCode::Left, Posicion::from(-1.0, 0.0)),
    ];
    let dir = dirs.iter().find(|(t, _)| teclas.just_pressed(*t));

    if let Some(direccion) = dir {
        let direccion = direccion.1;
        let muro = muros
            .iter()
            .find(|&&pos| pos == direccion + *posicion_jugador);

        if let Some(_) = muro {
            return;
        }

        let caja_cerca = cajas
            .iter()
            .find(|(_, pos, _)| **pos == direccion + *posicion_jugador);
        let lejos = cualquier_cosa
            .iter()
            .find(|pos| **pos == direccion * 2.0 + *posicion_jugador);

        if let Some(ref caja) = caja_cerca {
            // si la caja ha llegado al objetivo no se puede mover,
            // así que no nos movemos.
            if caja.2 .0 {
                return;
            }
        }

        //posibilidades:
        //      - (muro, _) -> no me muevo.
        //      - (caja, nada) -> me muevo y muevo la caja.
        //      - (caja_terminada, nada) -> no me muevo
        //      - (nada, _) -> me muevo.
        match (caja_cerca, lejos) {
            (Some(caja), None) => {
                let p1 = *posicion_jugador + direccion;
                let p2 = *caja.1 + direccion;
                com.entity(id_jugador).insert(p1);
                com.entity(caja.0).insert(p2);
            }
            (None, _) => {
                let p1 = *posicion_jugador + direccion;
                com.entity(id_jugador).insert(p1);
            }
            (_, _) => {}
        }
    }
}
