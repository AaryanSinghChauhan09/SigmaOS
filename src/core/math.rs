//! SigmaOS Custom Math Operations
//! Zero-dependency mathematical functions
//! Reduces dependency on standard library math functions
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


// (no_std only applicable at crate root - removed)

/// Custom absolute value for integers
pub fn abs_i32(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

/// Custom absolute value for i64
pub fn abs_i64(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

/// Custom minimum function
pub fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// Custom maximum function
pub fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Custom minimum function for usize
pub fn min_usize(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

/// Custom maximum function for usize
pub fn max_usize(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

/// Custom power function for integers
pub fn pow_i32(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    let mut base = base;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }

    result
}

/// Custom square root approximation using Newton's method
pub fn sqrt_f64(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN; // NaN for negative numbers
    }
    if x == 0.0 {
        return 0.0;
    }

    let mut guess = x / 2.0;
    let epsilon = 0.00001;

    loop {
        let new_guess = (guess + x / guess) / 2.0;
        if abs_f64(new_guess - guess) < epsilon {
            return new_guess;
        }
        guess = new_guess;
    }
}

/// Custom absolute value for f64
pub fn abs_f64(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Custom floor function
pub fn floor_f64(x: f64) -> f64 {
    if x >= 0.0 {
        x as i64 as f64
    } else {
        (x as i64 - 1) as f64
    }
}

/// Custom ceiling function
pub fn ceil_f64(x: f64) -> f64 {
    if x <= 0.0 {
        x as i64 as f64
    } else {
        (x as i64 + 1) as f64
    }
}

/// Custom round function
pub fn round_f64(x: f64) -> f64 {
    if x >= 0.0 {
        (x + 0.5) as i64 as f64
    } else {
        (x - 0.5) as i64 as f64
    }
}

/// Greatest Common Divisor using Euclidean algorithm
pub fn gcd(a: i32, b: i32) -> i32 {
    let mut a = abs_i32(a);
    let mut b = abs_i32(b);

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

/// Least Common Multiple
pub fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    abs_i32(a / gcd(a, b) * b)
}

/// Check if number is prime
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

/// Custom logarithm base 2 approximation
pub fn log2_f64(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    if x == 1.0 {
        return 0.0;
    }

    let mut result = 0.0;
    let mut x = x;

    while x >= 2.0 {
        x /= 2.0;
        result += 1.0;
    }

    // Add fractional part approximation
    result += (x - 1.0) / 2.0;

    result
}

/// Custom logarithm base 10 approximation
pub fn log10_f64(x: f64) -> f64 {
    log2_f64(x) / log2_f64(10.0)
}

/// Clamp value between min and max
pub fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Degrees to radians conversion
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * 3.14159265358979323846 / 180.0
}

/// Radians to degrees conversion
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / 3.14159265358979323846
}

/// Custom sine approximation using Taylor series
pub fn sin_f64(x: f64) -> f64 {
    let x = x % (2.0 * 3.14159265358979323846);
    let mut result = x;
    let mut term = x;
    let mut x_squared = x * x;

    for n in (1..=10).step_by(2) {
        term *= -x_squared / ((n + 1) * (n + 2)) as f64;
        result += term;
    }

    result
}

/// Custom cosine approximation using Taylor series
pub fn cos_f64(x: f64) -> f64 {
    let x = x % (2.0 * 3.14159265358979323846);
    let mut result = 1.0;
    let mut term = 1.0;
    let mut x_squared = x * x;

    for n in (2..=10).step_by(2) {
        term *= -x_squared / (n * (n + 1)) as f64;
        result += term;
    }

    result
}

// ==========================================
// Kernel-style checked arithmetic primitives
// ==========================================

pub fn saturating_add_i32(a: i32, b: i32) -> i32 {
    match a.checked_add(b) {
        Some(val) => val,
        None => if b > 0 { i32::MAX } else { i32::MIN },
    }
}

pub fn saturating_sub_i32(a: i32, b: i32) -> i32 {
    match a.checked_sub(b) {
        Some(val) => val,
        None => if b > 0 { i32::MIN } else { i32::MAX },
    }
}

pub fn checked_mul_i32(a: i32, b: i32) -> Option<i32> {
    a.checked_mul(b)
}

// =======================================================
// Invocation Frame and Stack Frame Alignment Verification
// =======================================================

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvocationFrame {
    pub rip: u64, // Instruction pointer
    pub rsp: u64, // Stack pointer
    pub rbp: u64, // Base pointer
    pub arg_count: u32,
    pub stack_canary: u64, // Buffer overflow protection canary
}

impl InvocationFrame {
    /// Validates System V x86_64 ABI 16-byte stack frame alignment
    pub fn is_x64_aligned(&self) -> bool {
        (self.rsp & 15) == 0
    }

    /// Validates ARM AAPCS 8-byte stack frame alignment
    pub fn is_arm_aligned(&self) -> bool {
        (self.rsp & 7) == 0
    }
}

pub struct FunctionInvocationManager {
    global_canary: u64,
}

impl FunctionInvocationManager {
    pub fn new(canary: u64) -> Self {
        Self { global_canary: canary }
    }

    /// Simulates a secure function invocation with stack canary verification
    pub fn secure_invoke_sim<F, R>(
        &self,
        frame: &InvocationFrame,
        f: F,
    ) -> Result<R, &'static str>
    where
        F: FnOnce() -> R,
    {
        // 1. Stack canary integrity verification (buffer overflow check)
        if frame.stack_canary != self.global_canary {
            return Err("Stack smash detected! Corrupted stack canary.");
        }

        // 2. Alignment boundary check (Ensure 16-byte aligned before calling on x86_64)
        if !frame.is_x64_aligned() {
            return Err("Alignment fault! Stack pointer is not 16-byte aligned.");
        }

        // 3. Execution under safe context
        Ok(f())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs_i32(-5), 5);
        assert_eq!(abs_i32(5), 5);
        assert_eq!(abs_i64(-100), 100);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_i32(3, 7), 3);
        assert_eq!(max_i32(3, 7), 7);
        assert_eq!(min_usize(10, 5), 5);
        assert_eq!(max_usize(10, 5), 10);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow_i32(2, 3), 8);
        assert_eq!(pow_i32(5, 0), 1);
        assert_eq!(pow_i32(3, 4), 81);
    }

    #[test]
    fn test_sqrt() {
        assert!((sqrt_f64(4.0) - 2.0).abs() < 0.001);
        assert!((sqrt_f64(9.0) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 7), 35);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(4));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp_i32(5, 0, 10), 5);
        assert_eq!(clamp_i32(-5, 0, 10), 0);
        assert_eq!(clamp_i32(15, 0, 10), 10);
    }

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_checked_arithmetic_overflows() {
        assert_eq!(saturating_add_i32(i32::MAX, 1), i32::MAX);
        assert_eq!(saturating_add_i32(i32::MIN, -1), i32::MIN);
        assert_eq!(saturating_sub_i32(i32::MIN, 1), i32::MIN);
        assert_eq!(saturating_sub_i32(i32::MAX, -1), i32::MAX);
        assert_eq!(checked_mul_i32(i32::MAX, 2), None);
    }

    #[test]
    fn test_invocation_frame_alignment() {
        let frame = InvocationFrame {
            rip: 0x401000,
            rsp: 0x7FFFF000, // 16-byte aligned
            rbp: 0x7FFFF040,
            arg_count: 0,
            stack_canary: 0xABCDEF,
        };
        assert!(frame.is_x64_aligned());
        assert!(frame.is_arm_aligned());

        let unaligned_frame = InvocationFrame {
            rip: 0x401000,
            rsp: 0x7FFFF004, // Not aligned to 16-byte boundary
            rbp: 0x7FFFF040,
            arg_count: 0,
            stack_canary: 0xABCDEF,
        };
        assert!(!unaligned_frame.is_x64_aligned());
        assert!(!unaligned_frame.is_arm_aligned()); // 7FFFF004 is not 8-byte aligned (0x7FFFF004 & 7 == 4)
    }

    #[test]
    fn test_secure_invocation_sim() {
        let manager = FunctionInvocationManager::new(0xABCDEF);

        let frame = InvocationFrame {
            rip: 0x401000,
            rsp: 0x7FFFF000, // Aligned
            rbp: 0x7FFFF040,
            arg_count: 0,
            stack_canary: 0xABCDEF, // Valid canary
        };

        let result = manager.secure_invoke_sim(&frame, || 42);
        assert_eq!(result, Ok(42));

        // Bad canary (stack smash)
        let corrupt_canary_frame = InvocationFrame {
            rip: 0x401000,
            rsp: 0x7FFFF000,
            rbp: 0x7FFFF040,
            arg_count: 0,
            stack_canary: 0x000000, // Corrupted canary
        };
        let result = manager.secure_invoke_sim(&corrupt_canary_frame, || 42);
        assert!(result.is_err());

        // Unaligned frame
        let unaligned_frame = InvocationFrame {
            rip: 0x401000,
            rsp: 0x7FFFF008, // 8-byte but not 16-byte aligned
            rbp: 0x7FFFF040,
            arg_count: 0,
            stack_canary: 0xABCDEF,
        };
        let result = manager.secure_invoke_sim(&unaligned_frame, || 42);
        assert!(result.is_err());
    }
}
