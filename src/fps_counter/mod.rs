use glfw;
pub struct FpsCounter {
    frame_counter: u32,
    last_time : f64,
    last_print_time: f64,
}

impl FpsCounter {
    pub fn new(glfw: &glfw::Glfw) -> FpsCounter {
        FpsCounter {
            frame_counter: 0,
            last_time: 0.0,
            last_print_time: glfw.get_time(),
        }
    }

    pub fn update(&mut self, glfw: &glfw::Glfw) {
        self.frame_counter += 1;
    } 

    pub fn print_if_necessary(&mut self, glfw: &glfw::Glfw) {
        let current_time = glfw.get_time();
        let elapsed_time = current_time - self.last_print_time;
        if elapsed_time >= 1.0 {
            let fps = self.frame_counter as f64 / elapsed_time;
            println!("FPS: {:.2}", fps);
            self.frame_counter = 0;
            self.last_print_time = current_time;
        }
    }
    
}