use beryllium::*;
use ogl33::load_gl_with;

const WINDOW_TITLE: &str = "TheDarkProject";

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).expect("Failed to get GL version");
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();


    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let _win = sdl
        .create_gl_window(win_args)
        .expect("Failed to create GL window");


    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
    }

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit { .. }, _) => break 'main_loop, _ => (),
            }
        }
    }
}