//Uses
use yamc_core::world::voxel::{AttributeRegistry, NameRegistry};

pub struct VoxelAttributeRegistries {
    next_id: u16,
    name_registry: NameRegistry,
}

impl VoxelAttributeRegistries {
    fn new() -> VoxelAttributeRegistries {
        VoxelAttributeRegistries {
            next_id: 0,
            name_registry: NameRegistry::new(),
        }
    }

    fn add_voxel_type(&mut self, name: &str) {
        self.name_registry.add(name, self.next_id).unwrap();
        self.next_id += 1;
    }
}

pub fn init_voxel_attribute_registries() -> VoxelAttributeRegistries {
    let mut registries = VoxelAttributeRegistries::new();

    registries.add_voxel_type("air");
    registries.add_voxel_type("grass");

    registries
}
