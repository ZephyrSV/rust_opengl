use crate::vao_n_vbo::Vertex;
pub trait Drawable{
    fn draw(&self);
}

pub struct Scene {
    pub meshes: Vec<tobj::Mesh>,
    pub vertices: Vec<Vertex>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(), //todo remove
            vertices: Vec::new(),
        }
    }
    pub fn load_obj(&mut self, path: &str) -> Result<(), tobj::LoadError> {
        let (models, _) = tobj::load_obj(path)?;
        for model in models {
            self.meshes.push(model.mesh);
        }
        for mesh in &self.meshes {
            println!("Mesh: {:?}", mesh);
        }
        for mesh in &self.meshes {
            for i in 0..mesh.positions.len() / 3 {
                let mut vertex = Vertex(
                    [
                        mesh.positions[i * 3],
                        mesh.positions[i * 3 + 1],
                        mesh.positions[i * 3 + 2],
                        1.0,
                    ],
                    [0.,0.,0.], //Normal
                    [0.,0.] //TexCoord
                );
                if mesh.normals.len() > 0 {
                    vertex.1 = [
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    ];
                }
                if mesh.texcoords.len() > 0 {
                    vertex.2 = [
                        mesh.texcoords[i*2],
                        mesh.texcoords[i*2+1],
                    ]
                }
                self.vertices.push(vertex);
            }
        }
        Ok(())
    }
}