use bevy::prelude::*;

enum BlockType {
    air,
    solid,
    liquid,
}

#[derive(Component)]
struct Voxel {
    id: u8,
    block_type: BlockType,
}

impl Voxel {
    fn generate_voxels() {
        // do some fun stuff here right
        let stone = Voxel {
            id: 0,
            block_type: BlockType::solid,
        };

        let void = Voxel {
            id: 1,
            block_type: BlockType::air,
        };

        let water = Voxel {
            id: 2,
            block_type: BlockType::liquid,
        };
    }
}
