extern crate nalgebra_glm as glm;

pub struct Node {
    pub position        : glm::Vec3,   // Where I should be in relation to my parent
    pub rotation        : glm::Vec3,   // How I should be rotated, around the X, the Y and the Z axes
    pub scale           : glm::Vec3,   // How I should be scaled
    pub reference_point : glm::Vec3,   // The point I shall rotate and scale about

    pub vao_id      : u32,             // What I should draw
    pub index_count : i32,             // How much of it there is to draw

    pub children: Vec<Node>, // Those I command
}

impl Node {

    pub fn new() -> Node {
        Node {
            position        : glm::zero(),
            rotation        : glm::zero(),
            scale           : glm::vec3(1.0, 1.0, 1.0),
            reference_point : glm::zero(),
            vao_id          : 0,
            index_count     : -1,
            children        : vec![],
        }
    }

    pub fn from_vao(vao_id: u32, index_count: i32) -> Node {
        Node {
            position        : glm::zero(),
            rotation        : glm::zero(),
            scale           : glm::vec3(1.0, 1.0, 1.0),
            reference_point : glm::zero(),
            vao_id,
            index_count,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child)
    }

    pub fn get_child(&mut self, index: usize) -> Node {
        self.children.get(index).unwrap()
    }

    pub fn get_n_children(&self) -> usize {
        self.children.len()
    }

    pub fn print(&self) {
        println!(
"SceneNode {{
    VAO:       {}
    Indices:   {}
    Children:  {}
    Position:  [{:.2}, {:.2}, {:.2}]
    Rotation:  [{:.2}, {:.2}, {:.2}]
    Reference: [{:.2}, {:.2}, {:.2}]
}}",
            self.vao_id,
            self.index_count,
            self.children.len(),
            self.position.x,
            self.position.y,
            self.position.z,
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
            self.reference_point.x,
            self.reference_point.y,
            self.reference_point.z,
        );
    }

}


// You can also use square brackets to access the children of a SceneNode
use std::ops::{Index, IndexMut};
impl Index<usize> for Node {
    type Output = Node;
    fn index(&self, index: usize) -> &Node {
        unsafe { self.children.get_unchecked(index) }
    }
}
impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Node {
        unsafe { self.children.get_unchecked_mut(index) }
    }
}
