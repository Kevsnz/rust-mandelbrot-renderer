use glium::uniforms::UniformsStorage;
use glium::Surface;

use crate::glium_data::Vertex;
use crate::viewport::Viewport;

pub struct Renderer {
    display: glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    viewport: Viewport,
}

impl Renderer {
    pub fn new(
        w: u32,
        h: u32,
        viewport: Viewport,
    ) -> (Self, glium::glutin::event_loop::EventLoop<()>) {
        let (event_loop, display) = init_display(w, h);
        let program = crate::shader::load_shader_programs(
            &display,
            crate::shader::VERTEX_PROGRAM_FILENAME,
            crate::shader::FRAGMENT_PROGRAM_SINGLE_FILENAME,
        );
        let (vertex_buffer, indices) = make_drawing_surface(&display);
        (
            Self {
                display,
                program,
                vertex_buffer,
                indices,
                viewport,
            },
            event_loop,
        )
    }

    pub fn render(self: &Renderer) {
        let (w, h) = self.display.get_framebuffer_dimensions();
        let ar = w as f64 / h as f64;

        let uniforms = UniformsStorage::new("ar", ar)
            .add("scale", self.viewport.scale)
            .add("offset_x", self.viewport.center_x)
            .add("offset_y", self.viewport.center_y);

        let mut frame = self.display.draw();
        frame
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }

    pub fn get_raw_frame(self: &mut Renderer) -> Vec<u8> {
        let image: glium::texture::RawImage2d<'_, u8> = self.display.read_front_buffer().unwrap();
        image.data.into_owned()
    }

    pub fn resize(&self, window_size: glium::glutin::dpi::PhysicalSize<u32>) {
        self.display.gl_window().resize(window_size.into());
    }
    pub fn redraw(self: &Renderer) {
        self.display.gl_window().window().request_redraw();
    }
    pub fn get_viewport(self: &mut Renderer) -> &mut Viewport {
        &mut self.viewport
    }
}

fn init_display(w: u32, h: u32) -> (glium::glutin::event_loop::EventLoop<()>, glium::Display) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::PhysicalSize::new(w, h))
        .with_title("Mandelbrot Set Explorer");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    (event_loop, display)
}

fn make_drawing_surface(
    display: &glium::Display,
) -> (glium::VertexBuffer<Vertex>, glium::index::NoIndices) {
    let vertex1 = Vertex {
        position: [-1.0, -1.0],
        coord: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        position: [1.0, -1.0],
        coord: [1.0, -1.0],
    };
    let vertex3 = Vertex {
        position: [-1.0, 1.0],
        coord: [-1.0, 1.0],
    };
    let vertex4 = Vertex {
        position: [1.0, 1.0],
        coord: [1.0, 1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    (vertex_buffer, indices)
}
