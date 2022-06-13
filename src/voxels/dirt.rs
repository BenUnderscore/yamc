//Uses
use super::voxel_uses::*;

pub fn register(builder: &mut VoxelSystemBuilder) {
    builder.add_voxel_type(
        "dirt",
        AppearanceAttribute::Solid(
            (122.0f / 255.0f, 74.0f / 255.0f, 15.0f / 255.0f)
        )
    );
}