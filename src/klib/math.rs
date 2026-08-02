// SigmaOS Custom Math Library
// Reduces dependency on predefined math functions

// (no_std only applicable at crate root - removed)

/// Custom absolute value function
pub fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Custom minimum function
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

/// Custom maximum function
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// Custom clamp function
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Custom power function for integers
pub fn pow(base: u32, exp: u32) -> u32 {
    let mut result = 1u32;
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

/// Custom square root approximation using binary search
pub fn sqrt(n: f64) -> f64 {
    if n < 0.0 {
        return f64::NAN; // Handle negative numbers
    }
    if n == 0.0 || n == 1.0 {
        return n;
    }

    let mut low = 0.0;
    let mut high = n;
    let mut mid = (low + high) / 2.0;

    while (mid * mid - n).abs() > 0.0001 {
        if mid * mid > n {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2.0;
    }

    mid
}

/// Custom logarithm base 2 using bit operations
pub fn log2(mut n: u32) -> u32 {
    if n == 0 {
        return u32::MAX; // Undefined for 0
    }

    let mut log = 0u32;
    while n > 1 {
        n >>= 1;
        log += 1;
    }

    log
}

/// Custom logarithm base 10
pub fn log10(mut n: u32) -> u32 {
    if n == 0 {
        return u32::MAX; // Undefined for 0
    }

    let mut log = 0u32;
    while n >= 10 {
        n /= 10;
        log += 1;
    }

    log
}

/// Greatest common divisor using Euclidean algorithm
pub fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

/// Least common multiple
pub fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a / gcd(a, b)) * b
}

/// Check if number is prime
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5u32;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

/// Custom rounding function
pub fn round(x: f64) -> i32 {
    if x >= 0.0 {
        (x + 0.5).floor() as i32
    } else {
        (x - 0.5).ceil() as i32
    }
}

/// Custom floor function
pub fn floor(x: f64) -> i32 {
    if x >= 0.0 {
        x as i32
    } else {
        if x == x as i32 as f64 {
            x as i32
        } else {
            (x as i32) - 1
        }
    }
}

/// Custom ceiling function
pub fn ceil(x: f64) -> i32 {
    if x == x as i32 as f64 {
        x as i32
    } else if x > 0.0 {
        (x as i32) + 1
    } else {
        x as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(5), 5);
        assert_eq!(abs(-5), 5);
        assert_eq!(abs(0), 0);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min(5, 10), 5);
        assert_eq!(max(5, 10), 10);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 3), 8);
        assert_eq!(pow(5, 0), 1);
        assert_eq!(pow(2, 10), 1024);
    }

    #[test]
    fn test_sqrt() {
        assert!((sqrt(4.0) - 2.0).abs() < 0.01);
        assert!((sqrt(9.0) - 3.0).abs() < 0.01);
        assert!((sqrt(16.0) - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(16), 4);
    }

    #[test]
    fn test_log10() {
        assert_eq!(log10(1), 0);
        assert_eq!(log10(10), 1);
        assert_eq!(log10(100), 2);
        assert_eq!(log10(1000), 3);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 7), 35);
        assert_eq!(lcm(12, 8), 24);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
    }

    #[test]
    fn test_round() {
        assert_eq!(round(3.4), 3);
        assert_eq!(round(3.6), 4);
        assert_eq!(round(-3.4), -3);
        assert_eq!(round(-3.6), -4);
    }

    #[test]
    fn test_floor_ceil() {
        assert_eq!(floor(3.7), 3);
        assert_eq!(floor(-3.7), -4);
        assert_eq!(ceil(3.2), 4);
        assert_eq!(ceil(-3.2), -3);
    }
}
