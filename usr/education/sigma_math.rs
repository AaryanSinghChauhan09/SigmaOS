/// SigmaOS: usr/education/sigma_math.rs
/// Embedded Computer Science and Data Science mathematical foundations.
/// Provides zero-allocation matrix math and big integer stubs.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaUsize = usize;

pub const MATRIX_MAX_DIM: SigmaUsize = 16;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub rows: SigmaUsize,
    pub cols: SigmaUsize,
    pub data: [f32; MATRIX_MAX_DIM * MATRIX_MAX_DIM],
}

impl Matrix {
    pub const fn empty() -> Self {
        Matrix {
            rows: 0,
            cols: 0,
            data: [0.0; MATRIX_MAX_DIM * MATRIX_MAX_DIM],
        }
    }
    
    pub fn get(&self, r: SigmaUsize, c: SigmaUsize) -> f32 {
        if r < self.rows && c < self.cols {
            self.data[r * MATRIX_MAX_DIM + c]
        } else {
            0.0
        }
    }
    
    pub fn set(&mut self, r: SigmaUsize, c: SigmaUsize, val: f32) {
        if r < self.rows && c < self.cols {
            self.data[r * MATRIX_MAX_DIM + c] = val;
        }
    }
}

/// Simple Linear Regression using Gradient Descent in no_std / zero allocation.
/// Fits y = m * x + c on matching slices. Returns (m, c).
pub fn ml_linear_regression_fit(x: &[f32], y: &[f32], epochs: usize, lr: f32) -> (f32, f32) {
    let mut m = 0.0f32;
    let mut c = 0.0f32;
    let n = x.len();
    if n == 0 { return (m, c); }

    for _ in 0..epochs {
        let mut dm = 0.0f32;
        let mut dc = 0.0f32;
        for i in 0..n {
            let prediction = m * x[i] + c;
            let error = prediction - y[i];
            dm += error * x[i];
            dc += error;
        }
        m -= (lr * dm) / (n as f32);
        c -= (lr * dc) / (n as f32);
    }
    (m, c)
}

/// Computes the dot product of two matrices. O(N^3) zero-allocation.
#[no_mangle]
pub unsafe extern "C" fn math_matrix_multiply(
    a: *const Matrix,
    b: *const Matrix,
    out: *mut Matrix
) -> i32 {
    if a.is_null() || b.is_null() || out.is_null() { return -1; }
    
    let a_mat = &*a;
    let b_mat = &*b;
    let o_mat = &mut *out;
    
    if a_mat.cols != b_mat.rows {
        return -22; // EINVAL (Dimension mismatch)
    }
    
    o_mat.rows = a_mat.rows;
    o_mat.cols = b_mat.cols;
    
    for r in 0..o_mat.rows {
        for c in 0..o_mat.cols {
            let mut sum = 0.0;
            for k in 0..a_mat.cols {
                sum += a_mat.get(r, k) * b_mat.get(k, c);
            }
            o_mat.set(r, c, sum);
        }
    }
    
    0
}
