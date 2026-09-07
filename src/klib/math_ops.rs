#![allow(dead_code)]
// SigmaOS Kernel Library - Math Operations Subsystem
// Inspired by Linux lib/math/ and FreeBSD kernel fixed-point math routines

/// Fast integer square root algorithm (Linux int_sqrt parity)
pub fn int_sqrt(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    let mut op = x;
    let mut res = 0u64;
    let mut one = 1u64 << 62; // Highest power of 4 <= u64::MAX

    while one > op {
        one >>= 2;
    }

    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res = (res >> 1) + one;
        } else {
            res >>= 1;
        }
        one >>= 2;
    }
    res
}

/// Aligns `val` up to the nearest multiple of `align` (must be power of two)
pub fn align_up(val: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (val + align - 1) & !(align - 1)
}

/// Aligns `val` down to the nearest multiple of `align` (must be power of two)
pub fn align_down(val: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    val & !(align - 1)
}

/// Checks if `val` is aligned to `align` boundary
pub fn is_aligned(val: usize, align: usize) -> bool {
    align_down(val, align) == val
}

/// Fixed-point 16.16 representation for kernel compute calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed1616(pub i32);

impl Fixed1616 {
    pub const SCALE: i32 = 65536;

    pub fn from_int(i: i32) -> Self {
        Self(i * Self::SCALE)
    }

    pub fn to_int(self) -> i32 {
        self.0 / Self::SCALE
    }

    pub fn add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    pub fn sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_sqrt() {
        assert_eq!(int_sqrt(0), 0);
        assert_eq!(int_sqrt(1), 1);
        assert_eq!(int_sqrt(4), 2);
        assert_eq!(int_sqrt(9), 3);
        assert_eq!(int_sqrt(16), 4);
        assert_eq!(int_sqrt(25), 5);
        assert_eq!(int_sqrt(100), 10);
    }

    #[test]
    fn test_alignments() {
        assert_eq!(align_up(0, 4096), 0);
        assert_eq!(align_up(1, 4096), 4096);
        assert_eq!(align_up(4096, 4096), 4096);

        assert_eq!(align_down(4095, 4096), 0);
        assert_eq!(align_down(4096, 4096), 4096);

        assert!(is_aligned(4096, 4096));
        assert!(!is_aligned(4097, 4096));
    }

    #[test]
    fn test_fixed_point() {
        let a = Fixed1616::from_int(5);
        let b = Fixed1616::from_int(3);
        assert_eq!(a.add(b).to_int(), 8);
        assert_eq!(a.sub(b).to_int(), 2);
    }
}
