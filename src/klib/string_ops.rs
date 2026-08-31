/// Custom string operations without std

pub fn custom_strlen(s: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

pub fn custom_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    unsafe {
        loop {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            if c1 == 0 {
                return 0;
            }
            i += 1;
        }
    }
}

pub fn custom_strncpy_safe_safe_safe(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    unsafe {
        loop {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
    }
    dest
}

pub fn custom_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

pub fn custom_memset(dest: *mut u8, c: u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = c;
        }
    }
    dest
}

pub fn pattern_match(s: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    if s.len() < pattern.len() {
        return None;
    }
    for i in 0..=(s.len() - pattern.len()) {
        let mut matches = true;
        for j in 0..pattern.len() {
            if s[i + j] != pattern[j] {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(i);
        }
    }
    None
}
