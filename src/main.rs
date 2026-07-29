//#![cfg_attr(not(debug_assertions), windows_subsystem = "linux")]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

pub mod glow;

use beryllium::*;
use ogl33::*;
use std::fs;
use std::ptr::null;
use beryllium::video::{GlContextFlags, GlSwapInterval};
use crate::glow::BufferType;

const VERTICES: &[glow::Vertex] = &[
    [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
    [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [-0.5, 0.5, 0.0, 0.2, 0.3, 0.4, 0.0, 1.0]
];


type TriIndices = [u32; 3];
const INDICES: [TriIndices; 2] = [[0, 1, 3], [1, 2, 3]];



fn main() -> () {

    let bitmap = {
        let mut file = fs::File::open("textures/obama.png"). unwrap();
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
        let mut bitmap = imagine::png::parse_png_rgba8(&bytes).unwrap().bitmap;
        bitmap.flip_scanlines();
        bitmap
    };



    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    let mut flags = GlContextFlags::default();

    if cfg!(debug_assertions) {
        flags |= GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();

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

    win.set_swap_interval(GlSwapInterval::Vsync).unwrap();

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    let vao = glow::VertexArray::new().unwrap();
    vao.bind();

    let _vbo = glow::Buffer::from_vertex_data(VERTICES).expect("");

    let ebo = glow::Buffer::new(BufferType::ElementArray).unwrap();

    unsafe {
        let slice: &[u8] = bytemuck::cast_slice(&INDICES);

        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            slice.len() as GLsizeiptr,
            slice.as_ptr() as *const _,
            GL_STATIC_DRAW,
        );
    }

    //const BORDER_COLOR: [f32; 4] = [ 1.0, 1.0, 0.0, 1.0 ];

    let mut textures = 0;
    unsafe {
        glGenTextures(1, &mut textures);
        glBindTexture(GL_TEXTURE_2D, textures);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);

        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGB as GLint,
            bitmap.width() as GLsizei,
            bitmap.height() as GLsizei,
            0,
            GL_RGB,
            GL_UNSIGNED_BYTE,
            bitmap.pixels().as_ptr().cast()
        );

        glGenerateMipmap(GL_TEXTURE_2D);

/*        glTexParameterfv(
            GL_TEXTURE_2D,
            GL_TEXTURE_BORDER_COLOR,
            BORDER_COLOR.as_ptr() as *const GLfloat
        );
*/
    }

    let vertex_shader_source= fs::read_to_string("shaders/shader.vert")
        .expect("Failed to read a shader file ");

    let fragment_shader_source = fs::read_to_string("shaders/shader.frag")
        .expect("Failed to read a shader file ");


    let shader_program =
        glow::ShaderProgram::from_shader_sources(&vertex_shader_source, &fragment_shader_source)
            .unwrap();

    shader_program.use_program();


    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        
        glow::clear_color(glow::Color::new(0.2, 0.3, 0.3, 1.0));

        glow::clear(glow::ClearBufferBit::ColorBuffer as isize);

        //glow::draw_arrays(glow::DrawMode::Triangles, 0, VERTICES.len().cast_signed());

        unsafe {
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, null());
        }
        win.swap_window();
        /*
        let mut max_attribs = 0;
        unsafe { glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut max_attribs) }
        println!("{}", max_attribs);*/
    }
}