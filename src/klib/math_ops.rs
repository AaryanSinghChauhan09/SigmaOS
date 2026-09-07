/// Custom math operations without std

pub fn custom_sqrt(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut i = 1;
    let mut result = 1;
    while result <= n {
        i += 1;
        result = i * i;
    }
    i - 1
}

pub fn custom_abs(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

pub fn custom_min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn custom_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn fixed_point_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

pub fn fixed_point_sub(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

pub fn bit_set(n: u32, bit: u8) -> u32 {
    n | (1 << bit)
}

pub fn bit_clear(n: u32, bit: u8) -> u32 {
    n & !(1 << bit)
}

pub fn bit_toggle(n: u32, bit: u8) -> u32 {
    n ^ (1 << bit)
}

pub fn bit_check(n: u32, bit: u8) -> bool {
    (n & (1 << bit)) != 0
}
