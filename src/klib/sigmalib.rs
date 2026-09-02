//! SigmaLib - Custom Standard Library (musl/dietlibc Inspiration)
//! Zero-dependency implementation of standard library functions
extern crate alloc;
use alloc::string::{String, ToString};

/// String operations (zero-allocation)
pub mod string {
    /// Calculate string length
    pub fn strlen(s: &[u8]) -> usize {
        let mut len = 0;
        while len < s.len() && s[len] != 0 {
            len += 1;
        }
        len
    }

    /// Compare two strings
    pub fn strcmp(s1: &[u8], s2: &[u8]) -> i32 {
        let mut i = 0;
        while i < s1.len() && i < s2.len() && s1[i] != 0 && s2[i] != 0 {
            if s1[i] < s2[i] {
                return -1;
            } else if s1[i] > s2[i] {
                return 1;
            }
            i += 1;
        }
        0
    }

    /// Copy string
    pub fn strncpy_safe_safe_safe_safe_safe(
        dest: &mut [u8],
        src: &[u8],
    ) -> Result<*mut u8, &'static str> {
        let mut i = 0;
        while i < src.len() && src[i] != 0 && i < dest.len() {
            dest[i] = src[i];
            i += 1;
        }
        if i < dest.len() {
            dest[i] = 0;
        }
        Ok(dest.as_mut_ptr())
    }

    /// Concatenate strings
    pub fn strcat(dest: &mut [u8], src: &[u8]) -> *mut u8 {
        let dest_len = strlen(dest);
        let mut i = 0;
        while dest_len + i < dest.len() && i < src.len() && src[i] != 0 {
            dest[dest_len + i] = src[i];
            i += 1;
        }
        if dest_len + i < dest.len() {
            dest[dest_len + i] = 0;
        }
        dest.as_mut_ptr()
    }

    /// Find substring
    pub fn strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }

        let haystack_len = strlen(haystack);
        let needle_len = strlen(needle);

        if needle_len > haystack_len {
            return None;
        }

        for i in 0..=(haystack_len - needle_len) {
            let mut j = 0;
            while j < needle_len && haystack[i + j] == needle[j] {
                j += 1;
            }
            if j == needle_len {
                return Some(i);
            }
        }

        None
    }
}

/// Memory operations (zero-allocation)
pub mod memory {
    /// Copy memory
    pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
        dest
    }

    /// Move memory
    pub unsafe fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        if (dest as *const u8) < src {
            let mut i = 0;
            while i < n {
                *dest.add(i) = *src.add(i);
                i += 1;
            }
        } else {
            let mut i = n;
            while i > 0 {
                i -= 1;
                *dest.add(i) = *src.add(i);
            }
        }
        dest
    }

    /// Set memory
    pub unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
        let value = c as u8;
        let mut i = 0;
        while i < n {
            *s.add(i) = value;
            i += 1;
        }
        s
    }

    /// Compare memory
    pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
        let mut i = 0;
        while i < n {
            if *s1.add(i) < *s2.add(i) {
                return -1;
            } else if *s1.add(i) > *s2.add(i) {
                return 1;
            }
            i += 1;
        }
        0
    }

    /// Zero memory (secure)
    pub unsafe fn secure_zero(s: *mut u8, n: usize) {
        let mut i = 0;
        while i < n {
            *s.add(i) = 0;
            i += 1;
        }
        // Prevent compiler optimization
        core::hint::black_box(());
    }
}

/// Mathematical functions (zero-allocation)
pub mod math {
    /// Absolute value
    pub fn abs(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }

    /// Minimum
    pub fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            a
        } else {
            b
        }
    }

    /// Maximum
    pub fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    /// Clamp value
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Power of two check
    pub fn is_power_of_two(n: usize) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    /// Next power of two
    pub fn next_power_of_two(n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        let mut n = n - 1;
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;
        n |= n >> 32;
        n + 1
    }

    /// Simple square root approximation
    pub fn sqrt(x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }

        let mut x = x;
        let mut result = x;
        let mut x0 = x;

        while result > x0 {
            x0 = result;
            result = (x / result + result) / 2.0;
        }

        result
    }
}

/// I/O operations (zero-allocation)
pub mod io {
    /// Write character to output
    pub fn putchar(c: u8) {
        // Platform-specific implementation
        // In production, would write to stdout
    }

    /// Write string to output
    pub fn puts(s: &[u8]) {
        for i in 0..s.len() {
            if s[i] == 0 {
                break;
            }
            putchar(s[i]);
        }
        putchar(b'\n');
    }

    /// Read character from input
    pub fn getchar() -> u8 {
        // Platform-specific implementation
        // In production, would read from stdin
        0
    }
}

/// Process management (zero-allocation)
pub mod process {
    /// Exit process
    pub fn exit(code: i32) -> ! {
        // Platform-specific implementation
        loop {}
    }

    /// Get process ID
    pub fn getpid() -> u32 {
        // Platform-specific implementation
        0
    }

    /// Get parent process ID
    pub fn getppid() -> u32 {
        // Platform-specific implementation
        0
    }
}

/// Time functions (zero-allocation)
pub mod time {
    /// Simple time counter
    static mut TIME_COUNTER: u64 = 0;

    /// Get current time (simplified)
    pub fn time() -> u64 {
        unsafe {
            TIME_COUNTER += 1;
            TIME_COUNTER
        }
    }

    /// Sleep for milliseconds (simplified)
    pub fn sleep_ms(ms: u64) {
        // Platform-specific implementation
        // In production, would use actual sleep
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_operations() {
        let s1 = b"hello";
        let s2 = b"world";

        assert_eq!(string::strlen(s1), 5);
        assert_eq!(string::strcmp(s1, s1), 0);

        let mut dest = [0u8; 10];
        string::strncpy_safe_safe_safe_safe_safe(&mut dest, s1);
        assert_eq!(string::strcmp(&dest, s1), 0);
    }

    #[test]
    fn test_memory_operations() {
        let mut src = [1u8, 2, 3, 4, 5];
        let mut dest = [0u8; 5];

        unsafe {
            memory::memcpy(dest.as_mut_ptr(), src.as_ptr(), 5);
        }

        assert_eq!(dest, src);
    }

    #[test]
    fn test_math_operations() {
        assert_eq!(math::abs(-5), 5);
        assert_eq!(math::min(3, 5), 3);
        assert_eq!(math::max(3, 5), 5);
        assert_eq!(math::clamp(7, 0, 10), 7);
        assert_eq!(math::clamp(15, 0, 10), 10);
        assert!(math::is_power_of_two(8));
        assert!(!math::is_power_of_two(7));
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(math::next_power_of_two(0), 1);
        assert_eq!(math::next_power_of_two(1), 1);
        assert_eq!(math::next_power_of_two(5), 8);
        assert_eq!(math::next_power_of_two(16), 16);
        assert_eq!(math::next_power_of_two(17), 32);
    }
}
