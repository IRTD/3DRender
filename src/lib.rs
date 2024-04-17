#![allow(unused)]

pub mod matrix;

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

pub struct Triangle {
    vertices: [Vertex; 3],
}

impl From<[[f64; 3]; 3]> for Triangle {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Triangle {
            vertices: value.map(|coords| Vertex::from(coords)),
        }
    }
}

pub struct Mesh {
    tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>) -> Self {
        Mesh { tris }
    }
}
