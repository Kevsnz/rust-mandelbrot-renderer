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
    let file = "video1.mp4";
    let mut set_encoder = SetEncoder::new(file, WIDTH, HEIGHT);

    let viewport = Viewport::new(0.0, 0.0, 0.5);
    let (mut renderer, event_loop) = renderer::Renderer::new(1680, 960, viewport);

    let start_pos = Point::new(renderer.get_viewport().center_x, renderer.get_viewport().center_y);
    let mut trajectory = Trajectory::new(start_pos, 1.0 / encoder::FRAME_RATE as f64);
    trajectory.add_move(-0.5, 0.0, 0.5, 0.1);
    trajectory.add_move(0.0, 0.25, 0.25, 0.5);
    trajectory.add_move(-0.25, -0.25, 0.15, 0.5);
    trajectory.add_move(-0.5, 0.25, 0.5, 0.5);
    trajectory.add_move(-1.0, 0.0, 0.15, 0.5);
    trajectory.add_move(-1.5, 0.0, 0.5, 0.1);

    set_encoder.open();
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

            let (new_center, new_scale) = trajectory.step(Point::new(
                renderer.get_viewport().center_x,
                renderer.get_viewport().center_y,
            ), renderer.get_viewport().scale);
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
