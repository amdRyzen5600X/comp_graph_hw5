#[macro_use]
extern crate glium;

use glium::glutin::surface::WindowSurface;
use glium::{Display, Surface};
use hw5::{ApplicationContext, State};

struct Application {
    pub vertex_buffer: glium::vertex::VertexBufferAny,
    pub program: glium::Program,
    pub camera: hw5::camera::CameraState,
    pub diffuse_tex: glium::Texture2d,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "hw5";

    fn new(display: &Display<WindowSurface>) -> Self {
        let (vertex_buffer, diffuse_texture) = hw5::load_wavefront(&display, include_bytes!("../uploads_files_3569627_maze-32x32.obj"), include_bytes!("../texture.jpg"));
        let program = program!(display,
            140 => {
                vertex: "
                    #version 140

                    uniform mat4 persp_matrix;
                    uniform mat4 view_matrix;

                    in vec3 position;
                    in vec3 normal;
                    in vec2 texture;

                    out vec3 v_position;
                    out vec3 v_normal;
                    out vec2 v_tex_coords;

                    void main() {
                        v_normal = normal;
                        v_tex_coords = texture;
                        v_position = position;
                        gl_Position = persp_matrix * view_matrix * vec4(position * 0.005, 1.0);
                    }
                ",

                fragment: "
                    #version 140

                    in vec3 v_normal;
                    in vec2 v_tex_coords;
                    in vec3 v_position;
                    out vec4 f_color;

                    uniform sampler2D diffuse_tex;

                    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                    float scalar_function(vec3 point){
                        float radius = 100.0;
                        float distance = length(point);
                        if (distance <= radius){
                            return 1.0;
                        } else {
                            return 0.1;
                        }
                    }

                    void main() {
                        float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
                        vec3 ambient_color = diffuse_color;
                        vec3 color = (0.3 + 0.7 * lum) * ambient_color * scalar_function(v_position);
                        f_color = vec4(color, 1.0);
                    }
                ",
            },

            110 => {
                vertex: "
                    #version 110

                    uniform mat4 persp_matrix;
                    uniform mat4 view_matrix;

                    attribute vec3 position;
                    attribute vec3 normal;
                    varying vec3 v_position;
                    varying vec3 v_normal;

                    void main() {
                        v_position = position;
                        v_normal = normal;
                        gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                    }
                ",

                fragment: "
                    #version 110

                    varying vec3 v_normal;

                    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                    void main() {
                        float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                        gl_FragColor = vec4(color, 1.0);
                    }
                ",
            },

            100 => {
                vertex: "
                    #version 100

                    uniform lowp mat4 persp_matrix;
                    uniform lowp mat4 view_matrix;

                    attribute lowp vec3 position;
                    attribute lowp vec3 normal;
                    varying lowp vec3 v_position;
                    varying lowp vec3 v_normal;

                    void main() {
                        v_position = position;
                        v_normal = normal;
                        gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                    }
                ",

                fragment: "
                    #version 100

                    varying lowp vec3 v_normal;

                    const lowp vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                    void main() {
                        lowp float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        lowp vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                        gl_FragColor = vec4(color, 1.0);
                    }
                ",
            },
        )
        .unwrap();

        let camera = hw5::camera::CameraState::new();

        Self {
            vertex_buffer,
            program,
            camera,
            diffuse_tex: diffuse_texture,
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: self.camera.get_perspective(),
            view_matrix: self.camera.get_view(),
            diffuse_tex: &self.diffuse_tex,
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();
        frame.finish().unwrap();
    }

    fn handle_window_event(
        &mut self,
        event: &glium::winit::event::WindowEvent,
        _window: &glium::winit::window::Window,
    ) {
        self.camera.process_input(&event);
    }

    fn update(&mut self) {
        self.camera.update();
    }
}

fn main() {
    State::<Application>::run_loop();
}
