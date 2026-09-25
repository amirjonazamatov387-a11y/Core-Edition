mod camera;
mod mesh;
mod state;

use std::sync::Arc;

use state::State;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Core Edition - Seed Prototype")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .expect("Failed to create window"),
    );

    log::info!("Initializing wgpu...");
    let mut app_state = pollster::block_on(State::new(window));

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == app_state.window().id() => {
                    if !app_state.input(event) {
                        match event {
                            WindowEvent::CloseRequested => elwt.exit(),
                            WindowEvent::Resized(physical_size) => {
                                app_state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { .. } => {
                                app_state.resize(app_state.window().inner_size());
                            }
                            WindowEvent::RedrawRequested => {
                                app_state.update();
                                match app_state.render() {
                                    Ok(()) => {}
                                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                        app_state.resize(app_state.size());
                                    }
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("Surface out of memory");
                                        elwt.exit();
                                    }
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout");
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::DeviceEvent { ref event, .. } => {
                    app_state.process_device_event(event);
                }
                Event::AboutToWait => {
                    app_state.window().request_redraw();
                }
                _ => {}
            }
        })
        .expect("Event loop error");
}
