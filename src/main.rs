#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

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

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("HumanGL")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;

    event_loop.run(move |event, elwt| {
        platform.handle_event(imgui.io_mut(), &window, &event);
        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            // Request a redraw when all events were processed.
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw()
            }
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    let ui = imgui.frame();

                    platform.prepare_render(ui, &window);

                    let draw_data = imgui.render();

                    if let Err(e) = unsafe { app.render(&window, draw_data) } {
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
                    if !imgui.io().want_capture_keyboard {
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

fn exit_app(app: &mut App, elwt: &EventLoopWindowTarget<()>) {
    elwt.exit();
    unsafe {
        app.destroy();
    }
}
