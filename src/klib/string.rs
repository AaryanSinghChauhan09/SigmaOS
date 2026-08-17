//! Custom String implementation for SigmaOS
//! Based on alloc::string::String for compatibility

extern crate alloc;
use alloc::string::String as AllocString;
use alloc::string::ToString as AllocToString;
use crate::klib::Vec;

pub use alloc::string::String;
pub use alloc::string::ToString;

pub fn format_int(mut num: u64) -> String {
    if num == 0 {
        return String::from("0");
    }
    
    let mut result = String::new();
    while num > 0 {
        let digit = (num % 10) as u8;
        result.push((b'0' + digit) as char);
        num /= 10;
    }
    
    // Reverse the string in place since we built it backwards
    let mut chars: Vec<char> = result.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        let temp = chars[i];
        chars[i] = chars[len - 1 - i];
        chars[len - 1 - i] = temp;
    }
    
    chars.into_iter().collect()
}
