use ogl33::*;

/*pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { glClearColor(r, g, b, a) }
}*/

pub fn clear_color(color: Color) {
   unsafe { glClearColor(color.r, color.g, color.b, color.a) }; 
}

struct Color {
    r: f32, 
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn create_color(r: f32, g: f32, b: f32, a: f32) -> Color { Color{ r, g, b, a} }
    
}