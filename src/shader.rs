const VERTEX_SHADER_SRC: &str = r#"

#version 400

in vec2 position;
in vec2 coord;
out vec2 pos;

void main() {
    pos = coord;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"

#version 400

uniform double ar;
uniform double offset_x;
uniform double offset_y;
uniform double scale;

in vec2 pos;
out vec4 color;

dvec2 csqr(dvec2 c1) {
    return dvec2(c1.x*c1.x - c1.y*c1.y, 2*c1.x*c1.y);
}

int calc(dvec2 c, double lim, int it) {
    dvec2 z = dvec2(0, 0);
    for (int i=0; i<it; i++) {
        z = csqr(z) + c;
        if (length(z) > lim) {
            return i;
        }
    }
    return -1;
}

vec3 get_color(float t) {
    vec3 colors[8];
    colors[0] = vec3(0.0, 0.0, 0.0);
    colors[1] = vec3(0.0, 0.0, 0.5);
    colors[2] = vec3(0.0, 0.0, 1.0);
    colors[3] = vec3(0.0, 1.0, 1.0);
    colors[4] = vec3(0.0, 1.0, 0.0);
    colors[5] = vec3(1.0, 0.0, 0.0);
    colors[6] = vec3(1.0, 1.0, 0.0);
    colors[7] = vec3(1.0, 1.0, 1.0);

    t *= 8.0;
    int a = int(floor(t));
    int b = int(ceil(t));
    return mix(colors[a], colors[b], t-a);
}

void main() {
    dvec2 c = dvec2(pos);
    c.x *= ar;
    c = c * scale + dvec2(offset_x, offset_y);
    int s = calc(c, 3, 200);
    if (s == -1) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else {
        float t = float(s) / 100.0;
        color = vec4(get_color(t), 1.0);
    }
}

"#;

pub fn get_shader_program(display: &glium::Display) -> glium::Program {
    glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap()
}
