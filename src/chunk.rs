use crate::voxels::voxels::Voxel;
use bevy::prelude::*;

pub const SIZE: usize = 16;

pub struct Chunk {
    pub voxels: [Voxel; SIZE * SIZE * SIZE],
    
}

impl Chunk {
    pub fn fetch_voxel(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                ..default()


            });

    }
}
