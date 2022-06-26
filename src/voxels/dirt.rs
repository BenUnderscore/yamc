//Uses
use super::voxel_uses::*;

pub fn register(builder: &mut VoxelSystemBuilder) {
    builder.add_voxel_type(
        "dirt",
        AppearanceAttribute::Solid(
            SolidModel {
                color: (122.0 / 255.0, 74.0 / 255.0, 15.0 / 255.0),
            }
        )
    );
}