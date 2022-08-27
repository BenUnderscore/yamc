//Uses
use yamc_core::world::voxel::{VoxelSystem, AttributeRegistry, AttributeRegistries, NameRegistry};
use yamc_core::render::voxel::{AppearanceAttribute, SolidColorCubeModel};
mod voxel_uses {
    pub(super) use super::{VoxelSystemBuilder, AppearanceAttribute, SolidColorCubeModel};
}

//Submodules
mod air;
mod dirt;

pub struct VoxelSystemBuilder {
    next_id: u16,
    name_registry: NameRegistry,
    appearance_registry: AttributeRegistry<AppearanceAttribute>,
}

impl VoxelSystemBuilder {
    fn new() -> VoxelSystemBuilder {
        VoxelSystemBuilder {
            next_id: 0,
            name_registry: NameRegistry::new(),
            appearance_registry: AttributeRegistry::new("AppearanceAttribute"),
        }
    }

    fn add_voxel_type(&mut self, name: &str, appearance: AppearanceAttribute) {
        self.name_registry.add(name, self.next_id).unwrap();
        self.appearance_registry.register(self.next_id, appearance);
        self.next_id += 1;
    }

    pub fn create_voxel_system(self) -> VoxelSystem {
        let mut  attribute_registries = AttributeRegistries::new();
        attribute_registries.add_registry(self.appearance_registry).unwrap();
        VoxelSystem::new(self.name_registry, attribute_registries)
    }
}

pub fn init_voxel_system() -> VoxelSystem {
    let mut builder = VoxelSystemBuilder::new();

    air::register(&mut builder);
    dirt::register(&mut builder);

    builder.create_voxel_system()
}