use crate::vao_n_vbo::{VBO, VAO};
use crate::set_attribute;
use crate::shader_n_program::{Shader, ShaderProgram, ShaderType};
use crate::{Vec3, Vec4, Mat3, Mat4, Point3};

type Pos = [f32; 3];
type Norm = [f32; 3];
type TexCoord = [f32; 2];

#[repr(C, packed)]
#[derive(Debug)]
pub struct Vertex(pub Pos, pub Norm, pub TexCoord);

pub trait Drawable{
    fn draw(&self);
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub vao: VAO,
    pub vbos: Vec<VBO>,
    pub shader_program: ShaderProgram,
    pub view: Mat4,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model: Mat4,
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
        }
    }
    pub fn load_obj(&mut self, path: &str) -> Result<(), tobj::LoadError> {
        let (models, _) = tobj::load_obj(path)?;
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            model: Mat4::identity(),
        };
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
    fn draw(&self) {
        unsafe {
            self.shader_program.apply();
            self.vao.bind();
            //
            let pos_attrib = self.shader_program.get_attrib_location("position").unwrap();
            let vao = &self.vao;
            set_attribute!(vao, pos_attrib, Vertex::0);
            let _ = self.shader_program.get_attrib_location("normal").map(|location| {
                set_attribute!(vao, location, Vertex::1);
            }).inspect_err(|x| println!("Error: {:?}", x));
            let _ = self.shader_program.get_attrib_location("texCoord").map(|location| {
                set_attribute!(vao, location, Vertex::2);
            }).inspect_err(|x| println!("Error: {:?}", x));
            //
            assert_eq!(self.vbos.len(), self.meshes.len());
            for i in 0..self.vbos.len() {
                self.vbos[i].bind();
                gl::DrawArrays(gl::TRIANGLES, 0, self.meshes[i].vertices.len() as i32);
            }
        }
    }
}