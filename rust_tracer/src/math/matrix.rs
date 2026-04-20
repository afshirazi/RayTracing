use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use num::traits::Inv;

use crate::math::difference_of_products;

// templating the size is probably overkill but it's fun and I want to do it :)

pub struct Matrix<const N: usize> {
    mat: [[f32; N]; N],
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> Matrix<N> {
    pub fn zero() -> Self {
        Self { mat: [[0.0; N]; N] }
    }

    pub fn new(mat: [[f32; N]; N]) -> Self {
        Self { mat }
    }
}

impl Matrix<4> {
    pub fn determinant(&self) -> f32 {
        todo!()
    }

    pub fn inverse(&self) -> Option<Self> {
        todo!()
    }
}

impl Matrix<3> {
    pub fn determinant(&self) -> f32 {
        let minor_12 = difference_of_products(self[1][1], self[2][2], self[1][2], self[2][1]);
        let minor_02 = difference_of_products(self[1][0], self[2][2], self[1][2], self[2][0]);
        let minor_01 = difference_of_products(self[1][0], self[2][1], self[1][1], self[2][0]);

        f32::mul_add(
            self[0][0],
            minor_12,
            difference_of_products(self[0][2], minor_01, -self[0][1], minor_02),
        )
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }

        let inv_det = det.inv();
        let mut r = [[0.0; 3]; 3];

        r[0][0] = inv_det * difference_of_products(self[1][1], self[2][2], self[1][2], self[2][1]);
        r[1][0] = inv_det * difference_of_products(self[1][2], self[2][0], self[1][0], self[2][2]);
        r[2][0] = inv_det * difference_of_products(self[1][0], self[2][1], self[1][1], self[2][0]);
        r[0][1] = inv_det * difference_of_products(self[0][2], self[2][1], self[0][1], self[2][2]);
        r[1][1] = inv_det * difference_of_products(self[0][0], self[2][2], self[0][2], self[2][0]);
        r[2][1] = inv_det * difference_of_products(self[0][1], self[2][0], self[0][0], self[2][1]);
        r[0][2] = inv_det * difference_of_products(self[0][1], self[1][2], self[0][2], self[1][1]);
        r[1][2] = inv_det * difference_of_products(self[0][2], self[1][0], self[0][0], self[1][2]);
        r[2][2] = inv_det * difference_of_products(self[0][0], self[1][1], self[0][1], self[1][0]);

        Some(Self::new(r))
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f32 {
        difference_of_products(self[0][0], self[1][1], self[0][1], self[1][0])
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det != 0.0 {
            let tmp_mat = Matrix::new([[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]]);
            let inv_mat = tmp_mat * det.inv();
            Some(inv_mat)
        } else {
            None
        }
    }
}

impl Matrix<1> {
    pub fn determinant(&self) -> f32 {
        self[0][0]
    }

    pub fn inverse(&self) -> Option<Self> {
        if self[0][0] == 0.0 {
            None
        } else {
            Some(Matrix::new([[self[0][0].inv()]]))
        }
    }
}

/// Generic matrix-vector multiply.
pub fn mul<R, T, const N: usize>(mat: &Matrix<N>, v: &T) -> R
where
    R: IndexMut<usize, Output = f32> + Default,
    T: Index<usize, Output = f32>,
{
    let mut ret = R::default();

    for i in 0..N {
        for j in 0..N {
            ret[i] = mat[i][j] * v[i];
        }
    }
    ret
}

/////////////// OPERATOR OVERLOADING /////////////////////

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f32; N];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.mat[idx]
    }
}

impl<const N: usize> Add for Matrix<N> {
    type Output = Matrix<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = self[i][j] + rhs[i][j];
            }
        }
        Matrix::new(mat)
    }
}

impl<const N: usize> Sub for Matrix<N> {
    type Output = Matrix<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = self[i][j] - rhs[i][j];
            }
        }
        Matrix::new(mat)
    }
}

impl<const N: usize> Mul<f32> for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = self[i][j] * rhs;
            }
        }
        Matrix::new(mat)
    }
}

impl<const N: usize> Mul<f32> for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = self[i][j] * rhs;
            }
        }
        Matrix::new(mat)
    }
}

impl<const N: usize> Div<f32> for &Matrix<N> {
    type Output = Matrix<N>;

    fn div(self, rhs: f32) -> Self::Output {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = self[i][j] / rhs;
            }
        }
        Matrix::new(mat)
    }
}
