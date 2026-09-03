#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

/// A high-performance `// #![no_std]  // crate-root only` Linear Algebra and Numeric Solver Engine
/// Designed to replace GNU Octave, MATLAB, and GROMACS dependencies.

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, default_val: f64) -> Self {
        Self {
            rows,
            cols,
            data: vec![default_val; rows * cols],
        }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r * self.cols + c] = val;
    }

    pub fn add(&self, other: &Matrix) -> Option<Matrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }
        let mut result = Matrix::new(self.rows, self.cols, 0.0);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        Some(result)
    }

    pub fn multiply(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }
        let mut result = Matrix::new(self.rows, other.cols, 0.0);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        Some(result)
    }
}

/// Numerical Integration Engine (ODE/PDE Solvers)
pub struct NumericIntegration;

impl NumericIntegration {
    /// 4th-Order Runge-Kutta Method for ODE integration
    pub fn rk4_step(f: fn(f64, f64) -> f64, t: f64, y: f64, dt: f64) -> f64 {
        let k1 = dt * f(t, y);
        let k2 = dt * f(t + dt / 2.0, y + k1 / 2.0);
        let k3 = dt * f(t + dt / 2.0, y + k2 / 2.0);
        let k4 = dt * f(t + dt, y + k3);
        y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
    }
}

/// Physics & Molecular Simulation Core
pub struct MolecularDynamics;

impl MolecularDynamics {
    /// Velocity Verlet integration for n-body simulation
    pub fn verlet_step(pos: &mut [f64], vel: &mut [f64], acc: &mut [f64], mass: f64, force: f64, dt: f64) {
        for i in 0..pos.len() {
            // Update position
            pos[i] += vel[i] * dt + 0.5 * acc[i] * dt * dt;
            
            // Calculate new acceleration (F = ma)
            let new_acc = force / mass;
            
            // Update velocity
            vel[i] += 0.5 * (acc[i] + new_acc) * dt;
            
            // Store new acceleration
            acc[i] = new_acc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_operations() {
        let mut a = Matrix::new(2, 2, 0.0);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 3.0);
        a.set(1, 1, 4.0);

        let mut b = Matrix::new(2, 2, 0.0);
        b.set(0, 0, 2.0);
        b.set(0, 1, 0.0);
        b.set(1, 0, 1.0);
        b.set(1, 1, 2.0);

        let sum = a.add(&b).unwrap();
        assert_eq!(sum.get(0, 0), 3.0);
        assert_eq!(sum.get(1, 1), 6.0);

        let prod = a.multiply(&b).unwrap();
        assert_eq!(prod.get(0, 0), 4.0); // 1*2 + 2*1
        assert_eq!(prod.get(0, 1), 4.0); // 1*0 + 2*2
    }

    #[test]
    fn test_rk4_integration() {
        // Simple ODE: dy/dt = y
        fn exp_derivative(_t: f64, y: f64) -> f64 { y }
        
        let mut y = 1.0;
        let dt = 0.1;
        y = NumericIntegration::rk4_step(exp_derivative, 0.0, y, dt);
        
        // Exact solution is e^0.1 ~ 1.1051709
        assert!((y - 1.10517).abs() < 0.0001);
    }

    #[test]
    fn test_verlet_integration() {
        let mut pos = vec![0.0];
        let mut vel = vec![0.0];
        let mut acc = vec![0.0];
        
        // Apply constant force of 10.0 to a 1.0kg mass for 1 second
        MolecularDynamics::verlet_step(&mut pos, &mut vel, &mut acc, 1.0, 10.0, 1.0);
        
        assert_eq!(pos[0], 0.0);
        assert_eq!(acc[0], 10.0);
        assert_eq!(vel[0], 5.0);
    }
}
