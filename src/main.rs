#! [cfg_attr(not(debug_assertions), windows_subsystem = "linux")]
pub mod glow;

use beryllium::*;
use ogl33::*;
use std::{fs};

type Vertex = [f32; 3];
type TriIndices = [u32; 3];

const VERTICES: [Vertex; 4] =
    [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];

const INDICES: [TriIndices; 2] = [[0, 1, 3], [1, 2, 3]];

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

    let vao = glow::VertexArray::new().unwrap();
    vao.bind();

    let vbo = glow::Buffer::new().unwrap();
    vbo.bind(glow::BufferType::Array);
    glow::Buffer::buffer_data(glow::BufferType::Array, bytemuck::cast_slice(&VERTICES) , GL_STATIC_DRAW);

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

    let ebo = glow::Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(glow::BufferType::ElementArray);
    glow::Buffer::buffer_data(
        glow::BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW
    );

    let vertex_shader_source= fs::read_to_string("shaders/shader.vert")
        .expect("Failed to read a shader file ");

    let fragment_shader_source = fs::read_to_string("shaders/shader.frag")
        .unwrap();

    let shader_program =
        glow::ShaderProgram::from_vert_frag(&vertex_shader_source, &fragment_shader_source).unwrap();

    shader_program.use_program();

    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    glow::polygon_mode(glow::PolygonMode::Line);

    //let egui_ctx = egui::Context::default();

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        glow::clear_color(glow::Color::new(0.2, 0.3, 0.3, 1.0));

        glow::clear(glow::ClearBufferBit::ColorBuffer as isize);

/*        let raw_input = egui::RawInput::default();
        let _full_output = egui_ctx.run(raw_input, |ctx| {
            egui::Window::new("Test").show(ctx, |ui| {
                ui.label("test");
            });
        } );
        egui::Window::new("Test").show(&egui_ctx, |ui| {
            ui.label("test");
        });*/

        unsafe {
            //glDrawArrays(GL_TRIANGLES, 0, 3);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }

        win.swap_window();
    }
}