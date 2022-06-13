//Uses
use yamc_core::world::voxel::{VoxelSystem, self};
use yamc_core::{event_loop, render, res, world};
use yamc_core::event_loop::winit;
use yamc_core::cgmath::One;
use yamc_core::cgmath;
use systems::Systems;
use std::sync::mpsc;
use std::thread;
use env_logger;
use std::time;

//Modules
mod voxels;
mod systems;

fn main() {
    env_logger::init();

    let (proxy_tx, proxy_rx) = mpsc::channel();

    thread::spawn(move || {
        let proxy = proxy_rx.recv().unwrap();
        run(proxy);
    });

    event_loop::run_event_loop(proxy_tx);
}

fn run(event_loop_proxy: event_loop::EventLoopProxy) {
    let device_event_rx = {
        let (tx, rx) = mpsc::channel();
        event_loop_proxy.register_device_event_sender(Some(tx));
        rx
    };

    let window_event_rx = {
        let (tx, rx) = mpsc::channel();
        event_loop_proxy.register_window_event_sender(Some(tx));
        rx
    };

    let mut systems = Systems::new();

    {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("Yet Another (Crappy) Minecraft Clone")
            .with_inner_size(winit::dpi::LogicalSize::<f32>::new(1024.0, 768.0))
            .with_resizable(false);

        event_loop_proxy.create_window(window_builder).unwrap();

        let systems = systems::Systems::init();

        //The game loop
        let mut duration_behind: time::Duration = Default::default();
        let mut last_instant = time::Instant::now();
        let mut should_end = false;

        while !should_end {
            for ev in device_event_rx.try_iter() {
                match ev.event {
                    winit::event::DeviceEvent::Key(input) => {
                        if let Some(key_code) = input.virtual_keycode {
                            match key_code {
                                winit::event::VirtualKeyCode::Escape => should_end = true,
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }

            for ev in window_event_rx.try_iter() {
                match ev {
                    event_loop::WindowEvent::CloseRequested => should_end = true,
                }
            }

            world.update();
            world.render();

            let new_instant = time::Instant::now();
            duration_behind += new_instant - last_instant;
            last_instant = new_instant;
        }
    }

    event_loop_proxy.exit();
}
