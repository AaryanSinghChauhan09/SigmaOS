/// SigmaOS: usr/education/sigma_scicomp.rs
/// Scientific Computing algorithms for embedded Data Science analysis.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

// ─── Numerical Integration ────────────────────────────────────────────────────

/// Simulates a mathematical function pointer in no_std.
/// We use an enum of known functions to avoid dynamic dispatch pointers if needed,
/// but a simple fn(f32)->f32 is standard rust.
type MathFunc = fn(f32) -> f32;

/// Performs numerical integration using Simpson's Rule.
/// Integrates function `f` from `a` to `b` using `n` even intervals.
#[no_mangle]
pub extern "C" fn scicomp_integrate_simpson(
    f: MathFunc,
    a: f32,
    b: f32,
    n: u32
) -> f32 {
    let intervals = if n % 2 != 0 { n + 1 } else { n }; // Must be even
    if intervals == 0 { return 0.0; }
    
    let h = (b - a) / (intervals as f32);
    let mut sum = f(a) + f(b);
    
    for i in 1..intervals {
        let x = a + (i as f32) * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }
    
    (h / 3.0) * sum
}

// ─── Example Function ─────────────────────────────────────────────────────────

/// An example function: f(x) = x^2 + 2x
pub fn example_quadratic(x: f32) -> f32 {
    x * x + 2.0 * x
}
