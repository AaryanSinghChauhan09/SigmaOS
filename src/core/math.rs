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

    for n in (2..=:10).step_by(2) {
        term *= -x_squared / (n * (n + 1)) as f64;
        result += term;
    }

    result
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
}
