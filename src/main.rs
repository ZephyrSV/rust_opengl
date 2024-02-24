extern crate gl;
extern crate glfw;

use glfw::Context; // Action Key
mod fps_counter;
mod drawable;
mod shader_n_program;
mod vao_n_vbo;
use fps_counter::FpsCounter;
use drawable::{Drawable, Scene};

type Mat4 = nalgebra::Matrix4<f32>;
type Vec3 = nalgebra::Vector3<f32>;
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

    let mut my_scene = Scene::new("shaders/vertex/auto-rotate.vert", "shaders/fragment/auto-rotate.frag");
    my_scene.load_obj("models/cube.obj").map_err(|e| println!("Error: {:?}", e)).unwrap();
    my_scene.view = nalgebra::Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
    );

    while !window.should_close() {
        // Process events
        glfw.poll_events();

        // Render
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            

            // Set up the model, view, and projection matrices
            let mut model_matrix = Mat4::new_translation(&Vec3::new(0.0, 0.0, 0.0));
            let angle = glfw.get_time() as f32;
            model_matrix *= Mat4::from_euler_angles(0.0,angle, angle);
            my_scene.meshes[0].model = model_matrix;

            my_scene.draw(&glfw);


        }

        // Swap buffers
        window.swap_buffers();

        // Calculate frame rate
        fps_counter.update();
        fps_counter.print_if_necessary(&glfw);
    }
}
