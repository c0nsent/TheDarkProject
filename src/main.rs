use beryllium::*;
use ogl33::*;

const WINDOW_TITLE: &str = "The Dark Project";

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")] {
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE).unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl.create_gl_window(win_args).expect("Failed to create a window");

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    unsafe {
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
    }

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        unsafe {
            glClearColor(0.2, 0.3, 0.3, 1.0);
        }
    }
}