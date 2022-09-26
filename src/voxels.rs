pub mod voxels {
    use bevy::prelude::*;

    pub enum VoxelType {
        Air,
        Dirt,
        Stone,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Voxel {
        color: Vec3,
        block_id: u8,
    }


    impl Voxel {
        fn voxel_generate(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })), 
                ..default()
            });
        }
    }
}
