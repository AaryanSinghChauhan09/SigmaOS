#![no_std]
#![allow(dead_code)]

/// SigmaOS In-Kernel Tensor Engine
/// A zero-allocation matrix math stub designed for edge inference models.

const MAX_DIM: usize = 32;

/// A simple 2D Tensor struct backed by a static array
#[derive(Copy, Clone)]
pub struct Tensor2D {
    pub rows: usize,
    pub cols: usize,
    pub data: [f32; MAX_DIM * MAX_DIM], // Simplified float array (in a real system, might be INT8 or fixed point)
}

impl Tensor2D {
    pub const fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: [0.0; MAX_DIM * MAX_DIM],
        }
    }
    
    pub fn get(&self, r: usize, c: usize) -> f32 {
        if r < self.rows && c < self.cols {
            self.data[r * self.cols + c]
        } else {
            0.0
        }
    }
    
    pub fn set(&mut self, r: usize, c: usize, val: f32) {
        if r < self.rows && c < self.cols {
            self.data[r * self.cols + c] = val;
        }
    }
}

/// General Matrix Multiply (GEMM): C = A * B
pub fn tensor_gemm(a: &Tensor2D, b: &Tensor2D, c: &mut Tensor2D) -> Result<(), &'static str> {
    if a.cols != b.rows {
        return Err("Matrix dimensions do not match for multiplication");
    }
    if c.rows != a.rows || c.cols != b.cols {
        return Err("Output matrix dimensions are incorrect");
    }
    
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            for k in 0..a.cols {
                sum += a.get(i, k) * b.get(k, j);
            }
            c.set(i, j, sum);
        }
    }
    
    Ok(())
}

/// Simple ReLU activation function
pub fn tensor_relu(t: &mut Tensor2D) {
    for i in 0..(t.rows * t.cols) {
        if t.data[i] < 0.0 {
            t.data[i] = 0.0;
        }
    }
}

#[no_mangle]
pub extern "C" fn sigma_tensor_demo() -> i32 {
    let mut a = Tensor2D::new(2, 2);
    a.set(0, 0, 1.0); a.set(0, 1, 2.0);
    a.set(1, 0, 3.0); a.set(1, 1, 4.0);
    
    let mut b = Tensor2D::new(2, 2);
    b.set(0, 0, 2.0); b.set(0, 1, 0.0);
    b.set(1, 0, 1.0); b.set(1, 1, 2.0);
    
    let mut c = Tensor2D::new(2, 2);
    
    match tensor_gemm(&a, &b, &mut c) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
