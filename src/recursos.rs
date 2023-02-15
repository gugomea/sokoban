use bevy::prelude::Resource;

#[derive(Resource, Default, Debug)]
pub struct CajasRestantes(usize);
