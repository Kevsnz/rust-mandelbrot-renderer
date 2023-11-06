use std::{fs::File, io::Read};

pub const VERTEX_PROGRAM_FILENAME: &str = "vertex.glsl";
pub const FRAGMENT_PROGRAM_SINGLE_FILENAME: &str = "fragment_single.glsl";
pub const FRAGMENT_PROGRAM_SSX4_FILENAME: &str = "fragment_ssx4.glsl";

pub fn load_shader_programs(display: &glium::Display, vertex_filename: &str, fragment_filename: &str) -> glium::Program {
    let mut vertex_shader_code = String::new();
    File::open(vertex_filename)
        .unwrap()
        .read_to_string(&mut vertex_shader_code)
        .unwrap();

    let mut fragment_shader_code = String::new();
    File::open(fragment_filename)
        .unwrap()
        .read_to_string(&mut fragment_shader_code)
        .unwrap();

    glium::Program::from_source(display, &vertex_shader_code, &fragment_shader_code, None).unwrap()
}
