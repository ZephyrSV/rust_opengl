extern crate gl;
extern crate glfw;

use glfw::Context; // Action Key
mod fps_counter;
mod drawable;
mod shader_n_program;
mod vao_n_vbo;
use fps_counter::FpsCounter;
use shader_n_program::{Shader, ShaderType, ShaderProgram};
use drawable::{Vertex, Drawable, Scene};

type Mat3 = nalgebra::Matrix3<f32>;
type Mat4 = nalgebra::Matrix4<f32>;
type Vec3 = nalgebra::Vector3<f32>;
type Vec4 = nalgebra::Vector4<f32>;
type Point3 = nalgebra::Point3<f32>;

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
    //enable backface culling
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    }

    // Initialize OpenGL
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let mut my_scene = Scene::new();
    my_scene.load_obj("models/cube.obj").map_err(|e| println!("Error: {:?}", e)).unwrap();


    let vertex_shader = Shader::new("shaders/vertex/auto-rotate.vert", ShaderType::Vertex)
            .expect("could not load vertex shader");
    let fragment_shader = Shader::new("shaders/fragment/auto-rotate.frag", ShaderType::Fragment)
            .expect("could not load fragment shader");

    let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader])
            .expect("could not create shader program");

    while !window.should_close() {
        // Process events
        glfw.poll_events();

        // Render
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.apply();

            let pos_attrib = shader_program.get_attrib_location("position").unwrap();
            let vao = &my_scene.vao;
            set_attribute!(vao, pos_attrib, Vertex::0);
            let _ = shader_program.get_attrib_location("normal").map(|location| {
                set_attribute!(vao, location, Vertex::1);
            }).inspect_err(|x| println!("Error: {:?}", x));
            let _ = shader_program.get_attrib_location("texCoord").map(|location| {
                set_attribute!(vao, location, Vertex::2);
            }).inspect_err(|x| println!("Error: {:?}", x));
            

            // Set up the model, view, and projection matrices
            let mut model_matrix = Mat4::new_translation(&Vec3::new(0.0, 0.0, 0.0));
            let angle = glfw.get_time() as f32;
            model_matrix *= Mat4::from_euler_angles(0.0,angle, angle);
            let view_matrix = nalgebra::Matrix4::look_at_rh(
                &Point3::new(0.0, 0.0, 3.0),
                &Point3::new(0.0, 0.0, 0.0),
                &Vec3::new(0.0, 1.0, 0.0),
            );
            let projection = nalgebra::Perspective3::new(800.0 / 600.0, 3.14 / 2.0, 0.1, 100.0);
            let normal_matrix = model_matrix.fixed_resize::<3, 3>(0.0).try_inverse().unwrap().transpose();


            let _ = shader_program.get_uniform_location("model").map(|x| {
                gl::UniformMatrix4fv(x as i32, 1, gl::FALSE, model_matrix.as_ptr());
            });
            let _ = shader_program.get_uniform_location("view").map(|x| {
                gl::UniformMatrix4fv(x as i32, 1, gl::FALSE, view_matrix.as_ptr());
            });
            let _ = shader_program.get_uniform_location("projection").map(|x| {
                gl::UniformMatrix4fv(x as i32, 1, gl::FALSE, projection.as_matrix().as_ptr());
            }); 
            let _ = shader_program.get_uniform_location("normalMatrix").map(|x| {
                gl::UniformMatrix3fv(x as i32, 1, gl::FALSE, normal_matrix.as_ptr());
            });

            let time = glfw.get_time() as f32;
            let _ = shader_program.get_uniform_location("time").map(|x| {
                    gl::Uniform1f(x as i32, time);
            });
            let light_pos = Vec3::new(-1.0, 0.4, 2.0);
            let _ = shader_program.get_uniform_location("lightPos").map(|x| {
                gl::Uniform3fv(x as i32, 1, light_pos.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));
            let light_color = Vec3::new(1.0, 1.0, 1.0);
            let _ = shader_program.get_uniform_location("lightColor").map(|x| {
                gl::Uniform3fv(x as i32, 1, light_color.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));
            let object_color = Vec3::new(1.0, 0.5, 0.31);
            let _ = shader_program.get_uniform_location("objectColor").map(|x| {
                gl::Uniform3fv(x as i32, 1, object_color.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));
            
            shader_program.apply();
            my_scene.draw();


        }

        // Swap buffers
        window.swap_buffers();

        // Calculate frame rate
        fps_counter.update();
        fps_counter.print_if_necessary(&glfw);
    }
}
