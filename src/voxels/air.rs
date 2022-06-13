//Uses
use super::voxel_uses::*;

pub fn register(builder: &mut VoxelSystemBuilder) {
    builder.add_voxel_type(
        "air",
        AppearanceAttribute::None,
    );
}