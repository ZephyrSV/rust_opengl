extern crate gl;
extern crate glfw;

use glfw::{Context}; // Action Key
mod fps_counter;
use fps_counter::FpsCounter;
mod shader_n_program;
use shader_n_program::{Shader, ShaderType, ShaderProgram};
mod vao_n_vbo;
use vao_n_vbo::{VAO, VBO, Vertex};

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}


fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks().unwrap();  
    let mut fps_counter = FpsCounter::new(&glfw);
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, _events) = glfw.create_window(800, 600, "OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Initialize OpenGL
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    #[rustfmt::skip]
    const VERTICES: [Vertex; 3] = [
        Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
        Vertex([0.0,   0.5], [0.0, 0.0, 1.0])
    ];

    let vertex_buffer = VBO::new(gl::ARRAY_BUFFER);
    unsafe {
        vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
    }
    
    let vertex_array = VAO::new();


    let vertex_shader = Shader::new("shaders/vertex/simple.vert", ShaderType::Vertex)
            .expect("could not load vertex shader");
    let fragment_shader = Shader::new("shaders/fragment/simple.frag", ShaderType::Fragment)
            .expect("could not load fragment shader");

    let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader])
            .expect("could not create shader program");

    while !window.should_close() {
        // Process events
        glfw.poll_events();

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.apply();

            let pos_attrib = shader_program.get_attrib_location("position").unwrap();
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = shader_program.get_attrib_location("color").unwrap();
            set_attribute!(vertex_array, color_attrib, Vertex::1);

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.apply();
            vertex_array.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);


        }

        // Swap buffers
        window.swap_buffers();

        // Calculate frame rate
        fps_counter.update();
        fps_counter.print_if_necessary(&glfw);
    }
}
