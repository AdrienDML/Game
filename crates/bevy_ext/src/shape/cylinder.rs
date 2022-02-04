use std::f32::consts::PI;

use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct Cylinder {
    pub height: f32,
    pub radius: f32,
    pub segments: u32,
}

impl From<Cylinder> for Mesh {
    fn from(cylinder: Cylinder) -> Self {
        let angle = 2. * PI / (cylinder.segments as f32);

        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        vertices.push([0., 0., 0.]);
        // Botom faces
        for i in 0..cylinder.segments {
            vertices.push([
                cylinder.radius * (angle * i as f32).cos(),
                0.0,
                cylinder.radius * (angle * i as f32).sin(),
            ]);
            indices.push(0);
            indices.push(i + 1);
            indices.push((i + 1) % cylinder.segments + 1);
        }
        // side faces
        for i in 0..cylinder.segments {
            indices.push(i + 1);
            indices.push((i + cylinder.segments) + 1);
            indices.push((i + 1) % cylinder.segments + 1);

            indices.push((i + cylinder.segments) + 1);
            indices.push((i + 1) % cylinder.segments + cylinder.segments + 1);
            indices.push((i + 1) % cylinder.segments + 1);
        }

        // Top faces
        for i in 0..cylinder.segments {
            vertices.push([
                cylinder.radius * (angle * i as f32).cos(),
                cylinder.height,
                cylinder.radius * (angle * i as f32).sin(),
            ]);
            indices.push(2 * cylinder.segments + 1);
            indices.push((i + 1) % cylinder.segments + cylinder.segments + 1);
            indices.push(i + cylinder.segments + 1);
        }

        vertices.push([0.0, cylinder.height, 0.0]);
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        for i in 0..vertices.len() {
            uvs.push([0., 0.]);
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
        mesh
    }
}
