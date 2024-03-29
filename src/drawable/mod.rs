use crate::vao_n_vbo::{VBO, VAO};
use crate::set_attribute;
use crate::shader_n_program::{Shader, ShaderProgram, ShaderType};
use crate::{Vec3, Mat4};

type Pos = [f32; 3];
type Norm = [f32; 3];
type TexCoord = [f32; 2];

#[repr(C, packed)]
#[derive(Debug)]
pub struct Vertex(pub Pos, pub Norm, pub TexCoord);

pub trait Drawable{
    fn draw(&self, glfw: &glfw::Glfw);
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub scale: f32,
    pub translate: Mat4,
    pub rotate: Mat4,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            scale: 1.0,
            translate: Mat4::identity(),
            rotate: Mat4::identity(),
        }
    }
    pub fn get_model_matrix(&self) -> Mat4 {
        self.translate * self.rotate * Mat4::new_scaling(self.scale)
    }
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub vao: VAO,
    pub vbos: Vec<VBO>,
    pub shader_program: ShaderProgram,
    pub view: Mat4,
    pub projection: Mat4,
}


impl Scene {
    pub fn new(vertex_shader_location: &str, fragment_shader_location: &str) -> Self {
        let vertex_shader = Shader::new(vertex_shader_location, ShaderType::Vertex)
                .expect("could not load vertex shader");
        let fragment_shader = Shader::new(fragment_shader_location, ShaderType::Fragment)
                .expect("could not load fragment shader");
        Self {
            meshes: Vec::new(), 
            vao: VAO::new(),
            vbos: Vec::new(),
            shader_program: ShaderProgram::new(&[vertex_shader, fragment_shader]).expect("could not create shader program"),
            view: Mat4::identity(),
            projection: nalgebra::Perspective3::new(800.0 / 600.0, 3.14 / 2.0, 0.1, 100.0).into(),
        }
    }
    pub fn load_obj(&mut self, path: &str) -> Result<(), tobj::LoadError> {
        let (models, _) = tobj::load_obj(path)?;
        let mut mesh = Mesh::new();
        for model in models.iter() {
            for i in 0..model.mesh.indices.len() {
                let i = model.mesh.indices[i] as usize;
                let normals: [f32; 3];
                if let Some(norm) = model.mesh.normals.get(i * 3..i * 3 + 3) {
                    normals = [norm[0], norm[1], norm[2]];
                } else {
                    normals = [0.0, 0.0, 0.0];
                }
                let tex_coords: [f32; 2];
                if let Some(tex) = model.mesh.texcoords.get(i * 2..i * 2 + 2) {
                    tex_coords = [tex[0], tex[1]];
                } else {
                    tex_coords = [0.0, 0.0];
                }
                let vertex = Vertex(
                    [
                        model.mesh.positions[i * 3],
                        model.mesh.positions[i * 3 + 1],
                        model.mesh.positions[i * 3 + 2],
                    ],
                    normals, //Normal
                    tex_coords, //TexCoords
                );
                mesh.vertices.push(vertex);
                mesh.indices.push(i as u32);
            }
        }
        println!("vertices count: {}", mesh.vertices.len());
        println!("vertices with zeroed normals: {:?}", mesh.vertices.iter().filter(|x| {
            let normal = x.1;
            normal == [0.0, 0.0, 0.0]
        }).count());

        if mesh.vertices.iter().all(|x| {let n = x.1; n == [0.0, 0.0, 0.0]}) {
            println!("All normals are zeroed, re-calculating normals");
            for i in 0..mesh.vertices.len() / 3 {
                let v0 = Vec3::new(
                    mesh.vertices[i*3].0[0],
                    mesh.vertices[i*3].0[1],
                    mesh.vertices[i*3].0[2],
                );
                let v1 = Vec3::new(
                    mesh.vertices[i*3+1].0[0],
                    mesh.vertices[i*3+1].0[1],
                    mesh.vertices[i*3+1].0[2],
                );
                let v2 = Vec3::new(
                    mesh.vertices[i*3+2].0[0],
                    mesh.vertices[i*3+2].0[1],
                    mesh.vertices[i*3+2].0[2],
                );
                let normal = (v1 - v0).cross(&(v2 - v0)).normalize();
                mesh.vertices[i*3].1 = [normal.x, normal.y, normal.z];
                mesh.vertices[i*3+1].1 = [normal.x, normal.y, normal.z];
                mesh.vertices[i*3+2].1 = [normal.x, normal.y, normal.z];
            }
        }
        let vbo = VBO::new(gl::ARRAY_BUFFER);
        unsafe {
            vbo.set_data(&mesh.vertices, gl::STATIC_DRAW);
        }
        self.meshes.push(mesh);
        self.vbos.push(vbo);
        Ok(())
    }
}

impl Drawable for Scene{
    fn draw(&self, glfw: &glfw::Glfw) {
        unsafe {
            // Bind the shader program and the VAO
            self.shader_program.apply();
            self.vao.bind();

            // Set up the vertex attributes
            let pos_attrib = self.shader_program.get_attrib_location("position").unwrap();
            let vao = &self.vao;
            set_attribute!(vao, pos_attrib, Vertex::0);
            let _ = self.shader_program.get_attrib_location("normal").map(|location| {
                set_attribute!(vao, location, Vertex::1);
            }).inspect_err(|x| println!("Error: {:?}", x));
            let _ = self.shader_program.get_attrib_location("texCoord").map(|location| {
                set_attribute!(vao, location, Vertex::2);
            }).inspect_err(|x| println!("Error: {:?}", x));

            // Set up the view, and projection matrices
            let _ = self.shader_program.get_uniform_location("view").map(|x| {
                gl::UniformMatrix4fv(x as i32, 1, gl::FALSE, self.view.as_ptr());
            });
            let _ = self.shader_program.get_uniform_location("projection").map(|x| {
                gl::UniformMatrix4fv(x as i32, 1, gl::FALSE, self.projection.as_ptr());
            }); 

            // Set up miscellaneous uniforms
            let time = glfw.get_time() as f32;
            let _ = self.shader_program.get_uniform_location("time").map(|x| {
                    gl::Uniform1f(x as i32, time);
            });
            let light_pos = Vec3::new(-1.0, 0.4, 2.0);
            let _ = self.shader_program.get_uniform_location("lightPos").map(|x| {
                gl::Uniform3fv(x as i32, 1, light_pos.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));
            let light_color = Vec3::new(1.0, 1.0, 1.0);
            let _ = self.shader_program.get_uniform_location("lightColor").map(|x| {
                gl::Uniform3fv(x as i32, 1, light_color.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));
            let object_color = Vec3::new(1.0, 0.5, 0.31);
            let _ = self.shader_program.get_uniform_location("objectColor").map(|x| {
                gl::Uniform3fv(x as i32, 1, object_color.as_ptr());
            }).map_err(|x| println!("Error: {:?}", x));

            // Draw the meshes and set their model/normal matrices
            assert_eq!(self.vbos.len(), self.meshes.len());
            let model_location = self.shader_program.get_uniform_location("model").unwrap();
            let normal_matrix_location = self.shader_program.get_uniform_location("normalMatrix");
            for i in 0..self.vbos.len() {
                self.vbos[i].bind();
                let model_matrix = self.meshes[i].get_model_matrix();
                gl::UniformMatrix4fv(model_location as i32, 1, gl::FALSE, model_matrix.as_ptr());
                let normal_matrix = model_matrix.fixed_resize::<3, 3>(0.0).try_inverse().unwrap().transpose();
                normal_matrix_location.as_ref().map(|x| gl::UniformMatrix3fv(*x as i32, 1, gl::FALSE, normal_matrix.as_ptr()));
                gl::DrawArrays(gl::TRIANGLES, 0, self.meshes[i].vertices.len() as i32);
            }
        }
    }
}