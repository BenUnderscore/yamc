//Uses
use yamc_core::{event_loop, render, res};
use yamc_core::event_loop::winit;
use yamc_core::cgmath::One;
use yamc_core::cgmath;
use std::sync::mpsc;
use std::thread;
use env_logger;
use std::time;

//Modules
mod voxels;

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

    let mut resource_system = res::ResourceSystem::new(std::path::PathBuf::from("./res/"));

    {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("Yet Another (Crappy) Minecraft Clone")
            .with_inner_size(winit::dpi::LogicalSize::<f32>::new(1024.0, 768.0))
            .with_resizable(false);

        event_loop_proxy.create_window(window_builder).unwrap();
        let mut renderer = render::RenderSystem::new(&event_loop_proxy, &mut resource_system);

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

            renderer.render(
                render::Camera {
                    position: cgmath::Vector3::new(0.0, 0.0, 0.0),
                    orientation: cgmath::Euler::new(0.0, 0.0, 0.0),
                    projection_matrix: cgmath::Matrix4::one(),
                }
            );

            let new_instant = time::Instant::now();
            duration_behind += new_instant - last_instant;
            last_instant = new_instant;
        }
    }

    event_loop_proxy.exit();
}
