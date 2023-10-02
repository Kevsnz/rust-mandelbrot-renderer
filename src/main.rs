mod encoder;
mod glium_data;
mod renderer;
mod shader;
mod trajectory;
mod viewport;

extern crate glium;

use encoder::SetEncoder;
use glium::glutin::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
use trajectory::{Point, Trajectory};
use viewport::Viewport;

const WIDTH: u32 = 1680;
const HEIGHT: u32 = 960;

fn main() {
    const START_X: f64 = 0.0;
    const START_Y: f64 = 0.0;
    const START_SCALE: f64 = 0.5;
    let file = "video1.mp4";
    
    let viewport = Viewport::new(START_X, START_Y, START_SCALE);
    let start_pos = Point::new(viewport.center_x, viewport.center_y);

    let mut trajectory = Trajectory::new(1.0 / encoder::FRAME_RATE as f64);
    trajectory.add_move(-0.5, 0.0, 0.5, 0.25);
    trajectory.add_move(-0.7, 0.15, 0.35, 0.25);
    trajectory.add_move(-1.0, 0.25, 0.15, 0.25);
    trajectory.add_move(-1.2, 0.32, 0.07, 0.25);
    trajectory.add_move(-1.24, 0.335, 0.02, 0.25);
    trajectory.add_move(-1.245, 0.335, 0.005, 0.25);
    trajectory.add_move(-1.247, 0.335, 0.001, 0.25);
    trajectory.add_move(-1.2475, 0.335, 0.0001, 0.25);
    trajectory.add_move(0.0, 0.0, 0.5, 0.25);
    trajectory.smooth(start_pos, viewport.scale);
    // return;
    
    let mut set_encoder = SetEncoder::new(file, WIDTH, HEIGHT);
    set_encoder.open();
    let (mut renderer, event_loop) = renderer::Renderer::new(1280, 960, viewport);
    event_loop.run(move |ev, _, control_flow| match ev {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(window_size) => {
                renderer.resize(window_size);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let ElementState::Pressed = input.state {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                        Some(VirtualKeyCode::NumpadAdd) => renderer.get_viewport().zoom_in(None),
                        Some(VirtualKeyCode::NumpadSubtract) => {
                            renderer.get_viewport().zoom_out(None)
                        }
                        Some(VirtualKeyCode::Left) => renderer.get_viewport().shift_left(None),
                        Some(VirtualKeyCode::Right) => renderer.get_viewport().shift_right(None),
                        Some(VirtualKeyCode::Up) => renderer.get_viewport().shift_up(None),
                        Some(VirtualKeyCode::Down) => renderer.get_viewport().shift_down(None),
                        Some(VirtualKeyCode::NumpadMultiply) => {
                            renderer.get_viewport().zoom_reset()
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        },
        Event::RedrawEventsCleared => {
            renderer.redraw();
        }
        Event::RedrawRequested(_) => {
            renderer.render();
            // set_encoder.add_frame(&renderer.get_raw_frame(), WIDTH, HEIGHT);

            let (new_center, new_scale) = trajectory.step();
            renderer
                .get_viewport()
                .set_center(new_center.x, new_center.y)
                .set_scale(new_scale);

            if trajectory.finished() {
                set_encoder.finalize();

                *control_flow = ControlFlow::Exit;
            }
        }
        _ => (),
    });
}
