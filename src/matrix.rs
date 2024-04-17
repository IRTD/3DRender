use std::{
    fmt::Display,
    ops::{Index, IndexMut, Mul, MulAssign},
};

use crate::Vertex;

#[derive(Default, Clone, Copy)]
pub struct Matrix4x4 {
    m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn scale(factor: f64) -> Self {
        let mut m = Matrix4x4::default();
        for i in 0..=3 {
            m[(i, i)] = factor;
        }
        m
    }

    pub fn x_rot(angle: f64) -> Self {
        let mut m = Matrix4x4::default();
        m[(0, 0)] = 1.0;
        m[(1, 1)] = angle.cos();
        m[(2, 2)] = angle.cos();
        m[(2, 1)] = angle.sin();
        m[(1, 2)] = -angle.sin();
        m
    }

    pub fn y_rot(angle: f64) -> Self {
        let mut m = Matrix4x4::default();
        m[(0, 0)] = angle.cos();
        m[(0, 2)] = angle.sin();
        m[(1, 1)] = 1.0;
        m[(2, 0)] = -angle.sin();
        m[(2, 2)] = angle.cos();
        m
    }

    pub fn projection_3d(aspect_ratio: f64, fov_rad: f64, far: f64, near: f64) -> Self {
        let mut m = Matrix4x4::default();
        m[(0, 0)] = aspect_ratio * fov_rad;
        m[(1, 1)] = fov_rad;
        m[(2, 2)] = far / (far - near);
        m[(3, 2)] = (-far * near) / (far - near);
        m[(2, 3)] = 1.0;
        m
    }

    pub fn z_rot(angle: f64) -> Self {
        let mut m = Matrix4x4::default();
        m[(0, 0)] = angle.cos();
        m[(0, 1)] = -angle.sin();
        m[(1, 0)] = angle.sin();
        m[(1, 1)] = angle.cos();
        m[(2, 2)] = 1.0;
        m
    }
}

impl From<[[f64; 4]; 4]> for Matrix4x4 {
    fn from(m: [[f64; 4]; 4]) -> Self {
        Matrix4x4 { m }
    }
}

impl From<Vertex> for Matrix4x4 {
    fn from(v: Vertex) -> Self {
        let mut m = Matrix4x4::default();
        m[(0, 0)] = v.x;
        m[(1, 1)] = v.y;
        m[(2, 2)] = v.z;
        m
    }
}

impl Into<Vertex> for Matrix4x4 {
    fn into(self) -> Vertex {
        Vertex::from([self[(0, 0)], self[(1, 1)], self[(2, 2)]])
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new = Matrix4x4::default();
        for i in 0..=3 {
            for j in 0..=3 {
                for k in 0..=3 {
                    new[(k, i)] += self[(i, j)] * rhs[(k, j)];
                }
            }
        }
        new
    }
}

impl MulAssign for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("   0.\t1.\t2.\t3.\n");
        for i in 0..=3 {
            let col = format!("{i}. ");
            s.push_str(&col);
            for j in 0..=3 {
                let p = format!("{}\t", self[(i, j)]);
                s.push_str(&p);
            }
            s.push('\n');
        }
        f.write_str(&s)
    }
}

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.m[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.m[index.0][index.1]
    }
}
