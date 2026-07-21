    #! [cfg_attr(not(debug_assertions), windows_subsystem = "linux")]
mod glow;

use beryllium::*;
use ogl33::*;
use std::{fs};

    type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

enum ShaderType {
    VertexShader,
    FragmentShader,
}

fn create_shader(shader_type: ShaderType, source: &str) -> Result<GLuint, String> {
    unsafe {
        let shader = glCreateShader(
            match shader_type {
                ShaderType::VertexShader => GL_VERTEX_SHADER,
                ShaderType::FragmentShader => GL_FRAGMENT_SHADER,
            }
        );

        if shader == 0 {
            return Err("Failed to create a shader".to_string());
        }

        glShaderSource(
            shader,
            1,
            &(source.as_bytes().as_ptr().cast()),
            &(source.len().try_into().unwrap())
        );

        glCompileShader(shader);

        let mut success = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            glGetShaderInfoLog(
                shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );

            return Err(String::from_utf8_lossy(&v).into_owned());
        }

        Ok(shader)
    }
}

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

        glClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW
        );

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);

        let vertex_shader_source= fs::read_to_string("src/shaders/shader.vert")
            .expect("Failed to read a shader file ");

        let vertex_shader =
            create_shader(ShaderType::VertexShader, &*vertex_shader_source)
            .expect("Failed to initialize vertex shader: ");

        let fragment_shader_source = fs::read_to_string("src/shaders/shader.frag")
            .unwrap();

        let fragment_shader =
            create_shader(ShaderType::FragmentShader, &*fragment_shader_source)
            .expect("Failed to initialize fragment shader: ");

        let shader_program = glCreateProgram();
        assert_ne!(shader_program, 0);
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);

        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        glUseProgram(shader_program);
    }

    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        unsafe {

            let color = create_color();

            glow::clear_color();

            glClear(GL_COLOR_BUFFER_BIT);

            glDrawArrays(GL_TRIANGLES, 0,3);
            win.swap_window();
        }
    }
}