#! [cfg_attr(not(debug_assertions), windows_subsystem = "linux")]
pub mod glow;

use beryllium::*;
use ogl33::*;
use std::{fs};
use crate::glow::*;


type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];



fn main() -> () {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    let win = sdl
        .create_gl_window(video::CreateWinArgs {
            title: "The Dark Project",
            width: 800,
            height: 600,
            allow_high_dpi: true,
            borderless: false,
            resizable: false,
        })
        .expect("Failed to create a window: ");

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    let vao = VertexArray::new().unwrap();
    vao.bind();

    let vbo = Buffer::new().unwrap();
    vbo.bind(BufferType::Array);
    Buffer::buffer_data(BufferType::Array, bytemuck::cast_slice(&VERTICES) , GL_STATIC_DRAW);

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }
    
    let vertex_shader_source= fs::read_to_string("src/shaders/shader.vert")
        .expect("Failed to read a shader file ");

    let fragment_shader_source = fs::read_to_string("src/shaders/shader.frag")
        .unwrap();

    let shader_program =
        ShaderProgram::from_vert_frag(&vertex_shader_source, &fragment_shader_source).unwrap();

    shader_program.use_program();

    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        clear_color(Color::new(0.2, 0.3, 0.3, 1.0));

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0,3);
        }

        win.swap_window();
    }
}