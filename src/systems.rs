//Uses
use yamc_core::res::ResourceSystem;
use yamc_core::world::CoreSystems;
use std::path::PathBuf;
use super::voxels;


pub struct Systems {
    pub resource: ResourceSystem,
    pub core_systems: CoreSystems,
}

impl Systems {

    pub fn new(resources_path: PathBuf) -> Systems {
        let resource_system = ResourceSystem::new(resources_path);
        let voxel_system = voxels::init_voxel_system();
        let render_system = render::RenderSystem::new(&event_loop_proxy, &mut resource_system)

        Systems {
            core_systems: Systems {
                resource: resource_system,
                core_systems: CoreSystems {
                    voxel: voxel_system,
                    render: Some(render_system),
                }
            }
        }
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) {
    }
}