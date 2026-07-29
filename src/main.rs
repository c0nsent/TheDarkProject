#! [cfg_attr(not(debug_assertions), windows_subsystem = "linux")]
pub mod glow;

use beryllium::*;
use ogl33::*;
use std::fs;

const VERTICES: &[glow::Vertex3D] = &[
    [-0.5, -0.5, 0.0], [1.0, 0.0, 0.0],
    [ 0.0,  0.5, 0.0], [0.0, 1.0, 0.0],
    [ 0.5, -0.5, 0.0], [0.0, 0.0, 1.0],
];


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

    let _vbo = glow::Buffer::from_vertex_data(VERTICES).expect("");

    let vertex_shader_source= fs::read_to_string("shaders/shader.vert")
        .expect("Failed to read a shader file ");

    let fragment_shader_source = fs::read_to_string("shaders/shader.frag")
        .expect("Failed to read a shader file ");

    let shader_program =
        glow::ShaderProgram::from_shader_sources(&vertex_shader_source, &fragment_shader_source).unwrap();

    //let uni_color = shader_program.get_uniform("uni_color").unwrap();

    shader_program.use_program();

    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    let mut is_negative = false;

    let mut offset = 0.0_f32;


    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }


        if offset >= 0.5 || offset <= -0.5 {
            is_negative = !is_negative;
        }

        let time = ((sdl.get_ticks() / 100000) as f32).sin();



        if is_negative {
            offset += 0.001;
        }
        else {
            offset -= 0.001;
        }



        shader_program.set_float("offset", offset).unwrap();


        glow::clear_color(glow::Color::new(0.2, 0.3, 0.3, 1.0));

        glow::clear(glow::ClearBufferBit::ColorBuffer as isize);

        glow::draw_arrays(glow::DrawMode::Triangles, 0, VERTICES.len().cast_signed());

        win.swap_window();
        /*
        let mut max_attribs = 0;
        unsafe { glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut max_attribs) }
        println!("{}", max_attribs);*/
    }
}