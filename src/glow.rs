use ogl33::*;


pub fn clear_color(color: Color) {
   unsafe { glClearColor(color.r, color.g, color.b, color.a) }; 
}

pub struct Color {
    r: f32, 
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Color{ r, g, b, a} }
    
}

pub struct VertexArray(pub GLuint);
impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao= 0;
        unsafe { glGenVertexArrays(1, &mut vao) };

        if vao != 0 {
            Some(Self(vao))
        } else {
          None
        }
    }

    pub fn bind(&self) -> () {
        unsafe { glBindVertexArray(self.0) }
    }

    pub fn clear_binding() -> () {
        unsafe { glBindVertexArray(0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = GL_ARRAY_BUFFER as isize,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe { glGenBuffers(1, &mut vbo); }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    pub fn bind(&self, buffer_type: BufferType) {
        unsafe { glBindBuffer(buffer_type as GLenum, self.0) }
    }


    pub fn buffer_data(buffer_type: BufferType, data: &[u8], usage: GLenum) {
/*        let mut coords = Vec::with_capacity(data.len() * 3);

        for vertex in data {
            coords.extend([vertex[0], vertex[1], 0.0])
        }
        
        let slice: &[u8] = bytemuck::cast_slice(&coords);*/

        unsafe {
            glBufferData(
                buffer_type as GLenum,
                data.len() as GLsizeiptr,
                data.as_ptr().cast(),
                usage,
            );
        }
    }
}


pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER as isize,
    Fragment = GL_FRAGMENT_SHADER as isize,
}


pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(shader_type: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(shader_type as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }


    pub fn set_source(&self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap())
            )
        }
    }


    pub fn compile(&self) {
        unsafe { glCompileShader(self.0) }
    }


    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };

        compiled == GLint::from(GL_TRUE)
    }


    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len)};
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast()
            );
            v.set_len(len_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }


    pub fn delete(self) {
        unsafe { glDeleteShader(self.0) }
    }


    pub fn from_source(shader_type: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(shader_type)
            .ok_or_else(|| "Couldn't allocate new shader".to_string())?;

        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }
}


pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {

    pub fn new() -> Option<Self> {
        let program = unsafe { glCreateProgram() };
        if program != 0 {
            Some(Self(program))
        } else {
            None
        }
    }


    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { glAttachShader(self.0, shader.0) }
    }


    pub fn link_program(&self) {
        unsafe { glLinkProgram(self.0) }
    }


    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == GLint::from(GL_TRUE)
    }


    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }


    pub fn use_program(&self) {
        unsafe { glUseProgram(self.0) }
    }


    pub fn delete(self) {
        unsafe { glDeleteProgram(self.0) }
    }


    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p =
            Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error:  {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error:  {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();

        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }
}


pub enum ClearBufferBit {
    ColorBuffer = GL_COLOR_BUFFER_BIT as isize,
    DepthBuffer = GL_DEPTH_BUFFER_BIT as isize,
    StencilBuffer = GL_STENCIL_BUFFER_BIT as isize,
}


pub fn clear(clear_buffers: isize) -> () {
    unsafe { glClear(clear_buffers as GLbitfield) }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Point = GL_POINT as isize,
    Line = GL_LINE as isize,
    Fill = GL_FILL as isize,
}

pub fn polygon_mode(mode: PolygonMode) {
    unsafe { glPolygonMode(GL_FRONT_AND_BACK, mode as GLenum) };
}

pub enum DrawMode{
    Points = GL_POINTS as isize,
    LineStrip = GL_LINE_STRIP as isize,
    LineLoop = GL_LINE_LOOP as isize,
    Lines = GL_LINES as isize,
    LineStripAdjacency = GL_LINE_STRIP_ADJACENCY as isize,
    LinesAdjacency = GL_LINES_ADJACENCY as isize,
    TriangleStrip = GL_TRIANGLE_STRIP as isize,
    TriangleFan = GL_TRIANGLE_FAN as isize,
    TriangleStripAdjacency = GL_TRIANGLE_STRIP_ADJACENCY as isize,
    TrianglesAdjacency = GL_TRIANGLES_ADJACENCY as isize,
    Triangles = GL_TRIANGLES as isize,
    //Patch загугли про GL_PATCHES
}

pub fn draw_arrays(mode: DrawMode, first: i32, count: isize) -> () {
    unsafe { glDrawArrays(mode as GLenum, first, count as GLsizei) }
}

pub type Vertex2D = [f32; 2];
