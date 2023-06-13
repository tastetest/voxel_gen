use bevy::prelude::*;

enum BlockType {
    Air(f32),
    Stone(f32),
    Dirt(f32)
}

#[derive(Component)]
struct Voxel {
    id: f32,
    block_type: BlockType,
}

// going to generate a chunk
fn chunk_gen(mut commands: Commands) {

    let voxel_gen = Vec3::new(0.2, 3.0, 4.0);

}
