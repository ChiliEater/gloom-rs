use std::ptr;

use crate::{byte_size_of_array, pointer_to_array};

use super::mesh::Mesh;

// == // Generate your VAO here
pub unsafe fn create_vao(mesh: &Mesh) -> u32 {
    // Generate array & store ID
    let mut vao_id: u32 = 0;
    gl::GenVertexArrays(1, &mut vao_id);

    // Bind VAO
    gl::BindVertexArray(vao_id);

    // Generate buffer & store ID
    let mut vbo_id: u32 = 0;
    gl::GenBuffers(1, &mut vbo_id);

    // Bind VBO
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);

    // Fill VBO
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&mesh.vertices),
        pointer_to_array(&mesh.vertices),
        gl::STATIC_DRAW,
    );

    // Setup VAP (clean this up?)
    let attribute_index = 0;
    gl::VertexAttribPointer(attribute_index, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

    // Enable VBO
    gl::EnableVertexAttribArray(attribute_index);

    // Generate index buffer & ID
    let mut ibo_id: u32 = 0;
    gl::GenBuffers(1, &mut ibo_id);

    // Bind IBO
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);

    // Fill IBO
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(&mesh.indices),
        pointer_to_array(&mesh.indices),
        gl::STATIC_DRAW,
    );

    let mut color_id: u32 = 0;
    gl::GenBuffers(1, &mut color_id);

    gl::BindBuffer(gl::ARRAY_BUFFER, color_id);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&mesh.colors),
        pointer_to_array(&mesh.colors),
        gl::STATIC_DRAW,
    );

    let color_attribute = 2;
    gl::VertexAttribPointer(color_attribute, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(color_attribute);

    vao_id
}

