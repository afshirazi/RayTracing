use std::ops::Mul;

use crate::math::{Vec3, matrix::Matrix};

#[derive(Default)]
pub struct Transform {
    mat: Matrix<4>,
    inv_mat: Matrix<4>,
}

impl Transform {
    pub fn new(mat: Matrix<4>) -> Self {
        let inv_mat = mat.inverse();
        let inv_mat = match inv_mat {
            Some(m) => m,
            None => Matrix::new([[f32::NAN; 4]; 4]),
        };

        Self { mat, inv_mat }
    }

    pub fn new_with_inv(mat: Matrix<4>, inv_mat: Matrix<4>) -> Self {
        Self { mat, inv_mat }
    }

    pub fn translate(delta: Vec3) -> Self {
        let mat = Matrix::new([
            [1.0, 0.0, 0.0, delta.x],
            [0.0, 1.0, 0.0, delta.y],
            [0.0, 0.0, 1.0, delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix::new([
            [1.0, 0.0, 0.0, -delta.x],
            [0.0, 1.0, 0.0, -delta.y],
            [0.0, 0.0, 1.0, -delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { mat, inv_mat }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mat = Matrix::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix::new([
            [x.recip(), 0.0, 0.0, 0.0],
            [0.0, y.recip(), 0.0, 0.0],
            [0.0, 0.0, z.recip(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { mat, inv_mat }
    }

    /// Rotates x axis `theta` radians
    pub fn rotate_x(theta: f32) -> Self {
        let mat = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), -theta.sin(), 0.0],
            [0.0, theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), theta.sin(), 0.0],
            [0.0, -theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { mat, inv_mat }
    }

    /// Rotates y axis `theta` radians
    pub fn rotate_y(theta: f32) -> Self {
        let mat = Matrix::new([
            [theta.cos(), 0.0, theta.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix::new([
            [theta.cos(), 0.0, -theta.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { mat, inv_mat }
    }

    /// Rotates z axis `theta` radians
    pub fn rotate_z(theta: f32) -> Self {
        let mat = Matrix::new([
            [theta.cos(), -theta.sin(), 0.0, 0.0],
            [theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix::new([
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { mat, inv_mat }
    }

    pub fn apply_point(&self, p: &Vec3) -> Vec3 {
        let xp =
            self.mat[0][0] * p.x + self.mat[0][1] * p.y + self.mat[0][2] * p.z + self.mat[0][3];
        let yp =
            self.mat[1][0] * p.x + self.mat[1][1] * p.y + self.mat[1][2] * p.z + self.mat[1][3];
        let zp =
            self.mat[2][0] * p.x + self.mat[2][1] * p.y + self.mat[2][2] * p.z + self.mat[2][3];
        let wp =
            self.mat[3][0] * p.x + self.mat[3][1] * p.y + self.mat[3][2] * p.z + self.mat[3][3];

        if wp == 1.0 {
            Vec3::new(xp, yp, zp)
        } else {
            Vec3::new(xp, yp, zp) / wp
        }
    }

    pub fn apply_vec(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.mat[0][0] * v.x + self.mat[0][1] * v.y + self.mat[0][2] * v.z,
            self.mat[1][0] * v.x + self.mat[1][1] * v.y + self.mat[1][2] * v.z,
            self.mat[2][0] * v.x + self.mat[2][1] * v.y + self.mat[2][2] * v.z,
        )
    }

    pub fn apply_norm(&self, n: &Vec3) -> Vec3 {
        Vec3::new(
            self.inv_mat[0][0] * n.x + self.inv_mat[1][0] * n.y + self.inv_mat[2][0] * n.z,
            self.inv_mat[0][1] * n.x + self.inv_mat[1][1] * n.y + self.inv_mat[2][1] * n.z,
            self.inv_mat[0][2] * n.x + self.inv_mat[1][2] * n.y + self.inv_mat[2][2] * n.z,
        )
    }
}

/////////////// OPERATOR OVERLOADING /////////////////////

impl Mul for &Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        Transform::new_with_inv(&self.mat * &rhs.mat, &rhs.inv_mat * &self.inv_mat)
    }
}
