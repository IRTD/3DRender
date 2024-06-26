#![allow(unused)]

pub mod matrix;
use anyhow::anyhow;
use matrix::Matrix4x4;

use std::{
    collections::VecDeque, fs::OpenOptions, io::BufRead, io::BufReader, ops::Index, path::PathBuf,
};

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

    pub fn apply_vec(&mut self, m: Matrix4x4) {
        m.vecmul(self);
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

    pub fn load_obj(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let p = path.into();
        let f = OpenOptions::new().read(true).write(false).open(p)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();
        let mut verts: VecDeque<Vertex> = VecDeque::new();
        let mut tris: Vec<Triangle> = Vec::new();
        // Collect all vertices and triangles
        for line in lines {
            let line = line?;
            if line.starts_with("#") {
                continue;
            }

            if line.starts_with("v") {
                let vertex = parse_obj_vertex(line)?;
                verts.push_back(vertex);
            } else if line.starts_with("f") {
                let tri = parse_obj_face(line, &verts)?;
                tris.push(tri);
            }
        }

        Ok(Mesh { tris })
    }
}

// Exmaple line: f 21 52 23
fn parse_obj_face(
    line: String,
    vertices: &impl Index<usize, Output = Vertex>,
) -> anyhow::Result<Triangle> {
    let idxs = &line.split_whitespace().collect::<Vec<&str>>()[1..];
    let mut tri = Triangle::default();
    for (i, idx) in idxs.iter().enumerate() {
        let v_idx: usize = (idx.parse::<usize>()?) - 1;
        tri.vertices[i] = vertices[v_idx];
    }
    Ok(tri)
}

// Example line: v 2.0 5.0 10.0
fn parse_obj_vertex(line: String) -> anyhow::Result<Vertex> {
    let coords = &line.split_whitespace().collect::<Vec<&str>>()[1..];
    let mut v: [Option<f64>; 3] = [None; 3];
    for (i, coord) in coords.into_iter().enumerate() {
        let value: f64 = coord.parse()?;
        v[i] = Some(value);
    }

    Ok(Vertex {
        x: v[0].unwrap(),
        y: v[1].unwrap(),
        z: v[2].unwrap(),
    })
}

#[derive(Clone)]
pub struct Camera {
    pub pos: Vertex,
    pub fov: f64,
}

impl Camera {
    pub fn new(pos: [f64; 3], fov: f64) -> Self {
        Camera {
            pos: Vertex::from(pos),
            fov,
        }
    }

    pub fn shift(&mut self, amount: f64, axis: Axis) {
        match axis {
            Axis::X => self.pos.x += amount,
            Axis::Y => self.pos.y += amount,
            Axis::Z => self.pos.z += amount,
        }
    }
}
