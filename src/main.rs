extern crate gl;
extern crate glfw;

use glfw::Context; // Action Key
mod fps_counter;
mod drawable;
mod shader_n_program;
mod vao_n_vbo;
use fps_counter::FpsCounter;
use shader_n_program::{Shader, ShaderType, ShaderProgram};
use vao_n_vbo::{VAO, VBO, Vertex};
use drawable::Scene;


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
    //glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    // Initialize OpenGL
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let mut my_scene = Scene::new();
    my_scene.load_obj("models/triangle.obj").unwrap();

    let vertex_buffer = VBO::new(gl::ARRAY_BUFFER);
    
    unsafe {
        vertex_buffer.set_data(&my_scene.vertices, gl::STATIC_DRAW);
    }
    for vertex in &my_scene.vertices {
        println!("{:?}", vertex);
    }
    let vertex_array = VAO::new();


    let vertex_shader = Shader::new("shaders/vertex/simple.vert", ShaderType::Vertex)
            .expect("could not load vertex shader");
    let fragment_shader = Shader::new("shaders/fragment/simple.frag", ShaderType::Fragment)
            .expect("could not load fragment shader");

    /*let vertex_shader = Shader::new("shaders/vertex/auto-rotate.vert", ShaderType::Vertex)
            .expect("could not load vertex shader");
    let fragment_shader = Shader::new("shaders/fragment/auto-rotate.frag", ShaderType::Fragment)
            .expect("could not load fragment shader");*/

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
            let normal_attrib = shader_program.get_attrib_location("normal").unwrap();
            set_attribute!(vertex_array, normal_attrib, Vertex::1);

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
