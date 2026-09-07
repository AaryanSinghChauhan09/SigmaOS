// SigmaOS Kernel Library Zero-Dependency Math Primitives

pub struct MathOps;

impl MathOps {
    pub fn clamp<T: Ord>(val: T, min: T, max: T) -> T {
        if val < min {
            min
        } else if val > max {
            max
        } else {
            val
        }
    }

    pub fn align_up(val: usize, alignment: usize) -> usize {
        debug_assert!(alignment.is_power_of_two());
        (val + alignment - 1) & !(alignment - 1)
    }

    pub fn align_down(val: usize, alignment: usize) -> usize {
        debug_assert!(alignment.is_power_of_two());
        val & !(alignment - 1)
    }

    pub fn is_aligned(val: usize, alignment: usize) -> bool {
        debug_assert!(alignment.is_power_of_two());
        (val & (alignment - 1)) == 0
    }
}
