#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::{Event, WindowEvent, StartCause};
use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use std::time::Instant;
use anyhow::Result;
use log::*;

use std::process;

use app::App;
use interactions::key::*;

mod app;
mod interactions;
mod model;
mod parser;
mod utils;
mod vulkan;

mod ui_system;

// ── Entry Point ───────────────────────────────────────────────────────────────


struct GeneralDataApp {
    window: Option<Window>,
    app: Option<App>,
    minimized: bool,
    last_frame: Instant,
}
//
// impl ApplicationHandler for GeneralDataApp {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         if self.window.is_none() {
//             let window_attributes = Window::default_attributes()
//                 .with_title("HumanGL")
//                 .with_inner_size(LogicalSize::new(1024, 768));
//
//             let window = event_loop
//                 .create_window(window_attributes)
//                 .expect("Failed to create window");
//
//             let app = unsafe { App::create(&window).expect("Failed to create app") };
//
//             self.window = Some(window);
//             self.app = Some(app);
//         }
//     }
//
//     fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
//         if let Some(app) = &mut self.app {
//             let now = Instant::now();
//             app.ui_system.imgui.io_mut().update_delta_time(now - self.last_frame);
//             self.last_frame = now;
//         }
//     }
//
//     fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
//         if let (Some(window), Some(App)) = (&self.window, &mut self.app) {
//             app.ui_system
//                 .platform
//                 .prepare_frame(app.ui_system.imgui.io_mut(), window)
//                 .expect("Failed to prepare frame");
//             window.request_redraw();
//         }
//     }
//
//     fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         window_id: WindowId,
//         event: WindowEvent
//     ) {
//
//     }
//
//
// }
//












fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new()?;
    let window_attributes = Window::default_attributes()
        .with_title("HumanGL")
        .with_inner_size(LogicalSize::new(1024, 768));

    let window = event_loop.create_window(window_attributes)?;
    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, elwt| {
        app.ui_system.platform.handle_event(app.ui_system.imgui.io_mut(), &window, &event);
        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                app.ui_system.imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            // Request a redraw when all events were processed.
            Event::AboutToWait => {
                app.ui_system.platform
                    .prepare_frame(app.ui_system.imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw()
            }
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    if let Err(e) = unsafe { app.render(&window) } {
                        error!("{e:#}");
                        process::exit(1);
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.resized = true;
                    }
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    exit_app(&mut app, elwt);
                }

                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    if !app.ui_system.imgui.io().want_capture_keyboard {
                        match handle_input(event.physical_key, &mut app.data) {
                            Ok(Signal::Nothing) => {}
                            Ok(Signal::Exit) => {
                                exit_app(&mut app, elwt);
                            }
                            Err(e) => {
                                error!("Error has ocurred while operating input {e}");
                            }
                        };
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}

fn exit_app(app: &mut App, elwt: &ActiveEventLoop) {
    elwt.exit();
    unsafe {
        app.destroy();
    }
}
