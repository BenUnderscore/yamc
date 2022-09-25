//Uses
use yamc_core::event_loop::EventLoopProxy;
use yamc_core::res::ResourceSystem;
use yamc_core::world::CoreSystems;
use yamc_core::world::voxel::{Voxel, VoxelArray};
use yamc_core::render;
use std::path::PathBuf;
use super::voxels;


pub struct Systems {
    pub resource: ResourceSystem,
    pub core_systems: CoreSystems,
}

impl Systems {

    pub fn new(resources_path: PathBuf, event_loop_proxy: &EventLoopProxy) -> Systems {
        let mut resource_system = ResourceSystem::new(resources_path);
        let voxel_system = voxels::init_voxel_system();
        let render_system = render::RenderSystem::new(&event_loop_proxy, &mut resource_system);

        let mut systems = Systems {
            core_systems: CoreSystems {
                voxel: voxel_system,
                render: Some(render_system),
            },
            resource: resource_system,
        };

        systems.test_init();
        systems
    }

    fn test_init(&mut self) {
        let voxel_system = &mut self.core_systems.voxel;
        let name_registry = voxel_system.name_registry();

        let air = Voxel { id: name_registry.find("air").unwrap(), data: 0 };
        let dirt = Voxel { id: name_registry.find("dirt").unwrap(), data: 0 };

        let test_chunk = VoxelArray::new(air);

        voxel_system.load_chunk(test_chunk, 0, 0, 0).unwrap();
    }

    pub fn update(&mut self) {
        if let Some(render_ref) = &mut self.core_systems.render {
            render_ref.update(&self.core_systems.voxel);
        }
        self.core_systems.voxel.reset_events();
    }

    pub fn render(&mut self) {
    }
}