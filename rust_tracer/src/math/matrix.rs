use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use num::traits::Inv;

use crate::math::difference_of_products;

// templating the size is probably overkill but it's fun and I want to do it :)
#[derive(Debug)]
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
        // https://www.geometrictools.com/Documentation/LaplaceExpansionTheorem.pdf
        let s0 = difference_of_products(self[0][0], self[1][1], self[1][0], self[0][1]);
        let s1 = difference_of_products(self[0][0], self[1][2], self[1][0], self[0][2]);
        let s2 = difference_of_products(self[0][0], self[1][3], self[1][0], self[0][3]);

        let s3 = difference_of_products(self[0][1], self[1][2], self[1][1], self[0][2]);
        let s4 = difference_of_products(self[0][1], self[1][3], self[1][1], self[0][3]);
        let s5 = difference_of_products(self[0][2], self[1][3], self[1][2], self[0][3]);

        let c0 = difference_of_products(self[2][0], self[3][1], self[3][0], self[2][1]);
        let c1 = difference_of_products(self[2][0], self[3][2], self[3][0], self[2][2]);
        let c2 = difference_of_products(self[2][0], self[3][3], self[3][0], self[2][3]);

        let c3 = difference_of_products(self[2][1], self[3][2], self[3][1], self[2][2]);
        let c4 = difference_of_products(self[2][1], self[3][3], self[3][1], self[2][3]);
        let c5 = difference_of_products(self[2][2], self[3][3], self[3][2], self[2][3]);

        difference_of_products(s0, c5, s1, c4)
            + difference_of_products(s2, c3, -s3, c2)
            + difference_of_products(s5, c0, s4, c1)
    }

    pub fn inverse(&self) -> Option<Self> {
        // taken from https://github.com/mmp/pbrt-v4/blob/master/src/pbrt/util/math.h#L1571
        let s0 = difference_of_products(self[0][0], self[1][1], self[1][0], self[0][1]);
        let s1 = difference_of_products(self[0][0], self[1][2], self[1][0], self[0][2]);
        let s2 = difference_of_products(self[0][0], self[1][3], self[1][0], self[0][3]);

        let s3 = difference_of_products(self[0][1], self[1][2], self[1][1], self[0][2]);
        let s4 = difference_of_products(self[0][1], self[1][3], self[1][1], self[0][3]);
        let s5 = difference_of_products(self[0][2], self[1][3], self[1][2], self[0][3]);

        let c0 = difference_of_products(self[2][0], self[3][1], self[3][0], self[2][1]);
        let c1 = difference_of_products(self[2][0], self[3][2], self[3][0], self[2][2]);
        let c2 = difference_of_products(self[2][0], self[3][3], self[3][0], self[2][3]);

        let c3 = difference_of_products(self[2][1], self[3][2], self[3][1], self[2][2]);
        let c4 = difference_of_products(self[2][1], self[3][3], self[3][1], self[2][3]);
        let c5 = difference_of_products(self[2][2], self[3][3], self[3][2], self[2][3]);

        let determinant = inner_product(&[s0, c5, -s1, c4, s2, c3, s3, c2, s5, c0, -s4, c1]);
        if determinant == 0.0 {
            return None;
        }
        let s = determinant.inv();

        let inv = [
            [
                s * inner_product(&[self[1][1], c5, self[1][3], c3, -self[1][2], c4]),
                s * inner_product(&[-self[0][1], c5, self[0][2], c4, -self[0][3], c3]),
                s * inner_product(&[self[3][1], s5, self[3][3], s3, -self[3][2], s4]),
                s * inner_product(&[-self[2][1], s5, self[2][2], s4, -self[2][3], s3]),
            ],
            [
                s * inner_product(&[-self[1][0], c5, self[1][2], c2, -self[1][3], c1]),
                s * inner_product(&[self[0][0], c5, self[0][3], c1, -self[0][2], c2]),
                s * inner_product(&[-self[3][0], s5, self[3][2], s2, -self[3][3], s1]),
                s * inner_product(&[self[2][0], s5, self[2][3], s1, -self[2][2], s2]),
            ],
            [
                s * inner_product(&[self[1][0], c4, self[1][3], c0, -self[1][1], c2]),
                s * inner_product(&[-self[0][0], c4, self[0][1], c2, -self[0][3], c0]),
                s * inner_product(&[self[3][0], s4, self[3][3], s0, -self[3][1], s2]),
                s * inner_product(&[-self[2][0], s4, self[2][1], s2, -self[2][3], s0]),
            ],
            [
                s * inner_product(&[-self[1][0], c3, self[1][1], c1, -self[1][2], c0]),
                s * inner_product(&[self[0][0], c3, self[0][2], c0, -self[0][1], c1]),
                s * inner_product(&[-self[3][0], s3, self[3][1], s1, -self[3][2], s0]),
                s * inner_product(&[self[2][0], s3, self[2][2], s0, -self[2][1], s1]),
            ],
        ];

        Some(Matrix::new(inv))
    }
}

// for now, just multiplies normally. TODO: look into propagating error https://pbr-book.org/4ed/Utilities/Mathematical_Infrastructure#Error-FreeTransformations
fn inner_product(nums: &[f32]) -> f32 {
    nums.chunks_exact(2)
        .map(|c| c[0] * c[1])
        .reduce(|a, b| a + b)
        .expect("Should not be empty")
}

impl Matrix<3> {
    pub fn determinant(&self) -> f32 {
        let minor_12 = difference_of_products(self[1][1], self[2][2], self[1][2], self[2][1]);
        let minor_02 = difference_of_products(self[1][0], self[2][2], self[1][2], self[2][0]);
        let minor_01 = difference_of_products(self[1][0], self[2][1], self[1][1], self[2][0]);

        f32::mul_add(
            self[0][0],
            minor_12,
            difference_of_products(self[0][2], minor_01, self[0][1], minor_02),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mat4_det() {
        let m = Matrix::new([
            [3.5, 8.0, 6.0, 8.0],
            [2.0, 4.0, 6.0, 7.0],
            [5.0, 1.5, 7.0, 0.0],
            [5.0, 5.5, 3.0, 1.0],
        ]);

        let expected = 163.5;
        let actual = m.determinant();

        assert_eq!(expected, actual, "Expected {expected}, but got {actual}");
    }

    #[test]
    fn test_mat4_inv() {
        let m = Matrix::new([
            [3.5, 0.0, 0.0, 0.0],
            [0.0, 4.0, 0.0, 0.0],
            [0.0, 0.0, 7.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let expected = Matrix::new([
            [3.5_f32.inv(), 0.0, 0.0, 0.0],
            [0.0, 4.0_f32.inv(), 0.0, 0.0],
            [0.0, 0.0, 7.0_f32.inv(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let actual = m.inverse().unwrap();

        assert!(
            eq_mat_approx(&expected, &actual),
            "Mismatched matrices: \nexpected = {:?}, \nactual = {:?}",
            expected,
            actual
        );
    }

    #[test]
    fn test_mat3_det() {
        let m = Matrix::new([[3.5, 8.0, 6.0], [2.0, 4.0, 6.0], [5.0, 1.5, 7.0]]);

        let expected = 92.5;
        let actual = m.determinant();

        assert_eq!(expected, actual, "Expected {expected}, but got {actual}");
    }

    #[test]
    fn test_mat3_inv() {
        let m = Matrix::new([[3.5, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 7.0]]);

        let expected = Matrix::new([
            [3.5_f32.inv(), 0.0, 0.0],
            [0.0, 4.0_f32.inv(), 0.0],
            [0.0, 0.0, 7.0_f32.inv()],
        ]);

        let actual = m.inverse().unwrap();

        assert!(
            eq_mat_approx(&expected, &actual),
            "Mismatched matrices: \nexpected = {:?}, \nactual = {:?}",
            expected,
            actual
        );
    }

    fn eq_mat_approx<const N: usize>(a: &Matrix<N>, b: &Matrix<N>) -> bool {
        for i in 0..N {
            for j in 0..N {
                if (a[i][j] - b[i][j]) > f32::EPSILON {
                    return false;
                }
            }
        }
        true
    }
}
