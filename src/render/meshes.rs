use std::ptr;

use tobj::Model;

use crate::render::vao::create_vao;

use super::mesh::Mesh;

pub struct Meshes {
    vaos: Vec<u32>,
    meshes: Vec<Mesh>,
}

impl Meshes {
    pub fn new() -> Meshes {
        Meshes {
            vaos: vec![],
            meshes: vec![],
        }
    }   

    pub fn add_all(&mut self, paths: &Vec<String>) {
        for path in paths {
            self.add(&path);
        }
    }

    pub fn add(&mut self, path: &String) {
        println!("Loading mesh \"{}\"...", path);
        let before = std::time::Instant::now();
        let (models, _materials)
            = tobj::load_obj(path,
                &tobj::LoadOptions{
                    triangulate: true,
                    single_index: true,
                    ..Default::default()
                }
            ).expect("Failed to load model");
        let after = std::time::Instant::now();
        println!("Done in {:.3}ms.", after.duration_since(before).as_micros() as f32 / 1e3);

        if models.is_empty() {
            panic!("Please use a model with at least one mesh!")
            // You could try merging the vertices and indices
            // of the separate meshes into a single mesh.
            // I'll leave that as an optional exercise. ;)
        }

        for model in models {
            let mesh = Mesh::from(model.to_owned().mesh, [1.0, 1.0, 1.0, 1.0]);
            self.meshes.push(mesh);
            println!("Loaded {} with {} points and {} triangles.",
                model.name,
                model.mesh.positions.len() / 3,
                model.mesh.indices.len() / 3,
            );
        }
    }

    pub fn generate_vaos(&mut self) {
        self.vaos = vec![];
        for mesh in &self.meshes {
            unsafe { self.vaos.push(create_vao(mesh)); }
        }
    }

}
