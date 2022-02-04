use std::f32::EPSILON;

use bevy::{
    math::{Mat4, Vec3, Vec3A},
    prelude::Mesh,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{PrimitiveTopology, VertexAttribute},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Line {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn transform(&self, transform: Mat4) -> Self {
        Self {
            origin: transform.transform_point3(self.origin),
            direction: transform.transform_vector3(self.direction),
        }
    }

    pub fn intersect_tri(&self, tri: &[Vec3A; 3]) -> Option<()> {
        // Determine the plan equation
        let dir: Vec3A = self.direction.into();
        let orig: Vec3A = self.origin.into();
        let vector_v0_to_v1: Vec3A = tri[1] - tri[0];
        let vector_v0_to_v2: Vec3A = tri[2] - tri[0];
        let p_vec: Vec3A = dir.cross(vector_v0_to_v2);
        let determinant: f32 = vector_v0_to_v1.dot(p_vec);
        if determinant < EPSILON {
            return None;
        }

        let determinant_inverse = 1.0 / determinant;

        let t_vec = orig - tri[0];
        let u = t_vec.dot(p_vec) * determinant_inverse;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q_vec = t_vec.cross(vector_v0_to_v1);
        let v = dir.dot(q_vec) * determinant_inverse;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // The distance between ray origin and intersection is t.
        let t: f32 = vector_v0_to_v2.dot(q_vec) * determinant_inverse;

        Some(())
    }

    pub fn intersect_plane() -> bool {
        unimplemented!()
    }

    pub fn line_mesh(&self, start: f32, end: f32) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let line_end = self.origin + end * self.direction;
        let line_start = self.origin + start * self.direction;

        let positions = vec![
            [line_start.x, line_start.y, line_start.z],
            [line_end.x, line_end.y, line_end.z],
        ];
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![[0.0, 0.0], [0.0, 0.0]]),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]),
        );
        return mesh;
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::*;

    // Triangle vertices to be used in a left-hand coordinate system

    #[test]
    fn raycast_triangle_mt() {
        let V0: Vec3A = Vec3A::new(1.0, -1.0, 2.0);
        let V1: Vec3A = Vec3A::new(1.0, 2.0, -1.0);
        let V2: Vec3A = Vec3A::new(1.0, -1.0, -1.0);
        let triangle = [V0, V1, V2];
        let ray = Line::new(Vec3::ZERO, Vec3::X);
        let result = ray.intersect_tri(&triangle);
        assert!(result.is_some());
    }
}
