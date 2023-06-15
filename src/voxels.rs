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
        fn generate_solids() {
            let stone = Voxel {
                id: 0,
                block_type: BlockType::solid,
            };
        }

        fn generate_air() {
            let void = Voxel {
                id: 1,
                block_type: BlockType::air,
            };
        }

        fn generate_liquids() {
            let water = Voxel {
                id: 2,
                block_type: BlockType::liquid,
            };
        }

        let block_type:BlockType = BlockType::solid; // this returns solid, obviously

        match block_type {
            BlockType::liquid => {
                generate_liquids();
                println!("this block is a liquid");
            },
            BlockType::air => {
                generate_air();
                println!("this block is air/void");
            },
            BlockType::solid => {
                generate_solids();
                println!("this block could be anything that is solid!");
            }
        }
    }
}
