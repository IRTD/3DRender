#![allow(unused)]

use std::ops::*;

use matrix::Matrix4x4;

pub mod matrix;

#[derive(Clone, Copy, Default)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn scale(&mut self, f: f64) {
        self.x += f;
        self.y += f;
        self.z += f;
    }
}

impl From<[f64; 3]> for Vertex {
    fn from(v: [f64; 3]) -> Self {
        Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl From<[[f64; 3]; 3]> for Triangle {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Triangle {
            vertices: value.map(|coords| Vertex::from(coords)),
        }
    }
}

impl Triangle {
    pub fn as_vertices(&self) -> Vec<Vertex> {
        self.vertices.map(|mat| mat.into()).to_vec()
    }

    pub fn apply_vec(&mut self, mat: Matrix4x4) {
        for m in &mut self.vertices {
            mat.vecmul(m);
        }
    }

    pub fn apply(&mut self, mat: Matrix4x4) {
        for m in &mut self.vertices {
            let v = Matrix4x4::from(*m);
            *m = (mat * v).into();
        }
    }

    pub fn scale_mul(&mut self, factor: f64, axis: Axis) {
        for v in &mut self.vertices {
            match axis {
                Axis::X => v.x *= factor,
                Axis::Y => v.y *= factor,
                Axis::Z => v.z *= factor,
            }
        }
    }

    pub fn scale_add(&mut self, factor: f64, axis: Axis) {
        for v in &mut self.vertices {
            match axis {
                Axis::X => v.x += factor,
                Axis::Y => v.y += factor,
                Axis::Z => v.z += factor,
            }
        }
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Default)]
pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>) -> Self {
        Mesh { tris }
    }

    pub fn as_vertices(&self) -> Vec<Vec<Vertex>> {
        self.tris.iter().map(|tri| tri.as_vertices()).collect()
    }

    pub fn apply_vec(&mut self, mat: Matrix4x4) {
        for tri in &mut self.tris {
            tri.apply_vec(mat);
        }
    }

    pub fn apply(&mut self, mat: Matrix4x4) {
        for tri in &mut self.tris {
            tri.apply(mat);
        }
    }
}
