use bevy::ecs::component::Component;

#[derive(Component, Default, Clone, Copy)]
pub struct Posicion {
    pub x: f32,
    pub y: f32,
}
impl Posicion {
    pub fn from(coords: (f32, f32)) -> Posicion {
        Posicion {
            x: coords.0,
            y: coords.1,
        }
    }
}

//#[derive(Component)]
//pub enum Movimiento {
//    Arriba,
//    Abajo,
//    Derecha,
//    Izquierda,
//}

#[derive(Component)]
pub struct Fijo; // marker component: componenete para ayudar a identificar entidades.

#[derive(Component)]
pub struct Movible; // marker component: componenete para ayudar a identificar entidades.

#[derive(Component)]
pub struct Terminado(pub bool);
