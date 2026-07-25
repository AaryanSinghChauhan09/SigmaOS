//! SigmaOS Uniq Compatibility
//! Filter adjacent matching lines (uniq command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Uniq options
#[repr(C)]
pub struct UniqOptions {
    pub count: SigmaBool,
    pub repeated: SigmaBool,
    pub unique: SigmaBool,
    pub ignore_case: SigmaBool,
    pub skip_fields: SigmaU32,
    pub skip_chars: SigmaU32,
}

/// Uniq state
const MAX_LINES: usize = 10000;
const MAX_LINE_LENGTH: usize = 1024;

static mut UNIQ_LINES: [[u8; MAX_LINE_LENGTH]; MAX_LINES] = [[0; MAX_LINE_LENGTH]; MAX_LINES];
static mut UNIQ_LINE_COUNT: SigmaU32 = 0;
static mut UNIQ_INITIALIZED: SigmaBool = false;

/// Initialize uniq
#[no_mangle]
pub unsafe extern "C" fn uniq_init() -> SigmaI32 {
    UNIQ_INITIALIZED = true;
    UNIQ_LINE_COUNT = 0;
    
    0 // Success
}

/// Add line to uniq
#[no_mangle]
pub unsafe extern "C" fn uniq_add_line(line: *const u8) -> SigmaI32 {
    if !UNIQ_INITIALIZED || line.isnull() || UNIQ_LINE_COUNT >= MAX_LINES as SigmaU32 {
        return -1;
    }
    
    for i in 0..MAX_LINE_LENGTH - 1 {
        let byte = *line.add(i);
        if byte == 0 || byte == b'\n' { break; }
        UNIQ_LINES[UNIQ_LINE_COUNT as usize][i] = byte;
    }
    
    UNIQ_LINE_COUNT += 1;
    
    0 // Success
}

/// Filter unique lines
#[no_mangle]
pub unsafe extern "C" fn uniq_execute(
    output: *mut u8,
    max_output: SigmaU32,
    options: UniqOptions,
) -> SigmaU32 {
    if !UNIQ_INITIALIZED || output.isnull() || UNIQ_LINE_COUNT == 0 {
        return 0;
    }
    
    let mut output_idx = 0;
    let mut current_idx = 0;
    
    while current_idx < UNIQ_LINE_COUNT as usize {
        let mut count = 1;
        let mut next_idx = current_idx + 1;
        
        // Count adjacent duplicates
        while next_idx < UNIQ_LINE_COUNT as usize {
            let matches = if options.ignore_case {
                compare_ignore_case(current_idx, next_idx) == 0
            } else {
                compare_lines(current_idx, next_idx) == 0
            };
            
            if matches {
                count += 1;
                next_idx += 1;
            } else {
                break;
            }
        }
        
        // Determine if this line should be output
        let should_output = if options.repeated {
            count > 1
        } else if options.unique {
            count == 1
        } else {
            true
        };
        
        if should_output {
            // Add count if requested
            if options.count {
                // Convert count to string manually
                let mut temp_count = count;
                let mut count_digits = [0u8; 20];
                let mut digit_idx = 0;
                
                if temp_count == 0 {
                    count_digits[0] = b'0';
                    digit_idx = 1;
                } else {
                    while temp_count > 0 && digit_idx < 20 {
                        count_digits[digit_idx] = b'0' + ((temp_count % 10) as u8);
                        temp_count /= 10;
                        digit_idx += 1;
                    }
                }
                
                // Output digits in reverse order
                for i in (0..digit_idx).rev() {
                    if output_idx >= max_output as usize {
                        return output_idx;
                    }
                    *output.add(output_idx) = count_digits[i];
                    output_idx += 1;
                }
                
                if output_idx < max_output as usize {
                    *output.add(output_idx) = b' ';
                    output_idx += 1;
                }
            }
            
            // Add line
            for i in 0..MAX_LINE_LENGTH {
                if output_idx >= max_output as usize {
                    return output_idx;
                }
                
                let byte = UNIQ_LINES[current_idx][i];
                if byte == 0 { break; }
                
                *output.add(output_idx) = byte;
                output_idx += 1;
            }
            
            if output_idx < max_output as usize {
                *output.add(output_idx) = b'\n';
                output_idx += 1;
            }
        }
        
        current_idx = next_idx;
    }
    
    output_idx
}

/// Clear lines
#[no_mangle]
pub unsafe extern "C" fn uniq_clear() -> SigmaI32 {
    if !UNIQ_INITIALIZED {
        return -1;
    }
    
    UNIQ_LINE_COUNT = 0;
    
    0 // Success
}

/// Get line count
#[no_mangle]
pub unsafe extern "C" fn uniq_get_line_count() -> SigmaU32 {
    UNIQ_LINE_COUNT
}

unsafe fn compare_lines(idx1: usize, idx2: usize) -> SigmaI32 {
    for i in 0..MAX_LINE_LENGTH {
        let b1 = UNIQ_LINES[idx1][i];
        let b2 = UNIQ_LINES[idx2][i];
        
        if b1 == 0 && b2 == 0 {
            return 0;
        }
        if b1 == 0 {
            return -1;
        }
        if b2 == 0 {
            return 1;
        }
        
        if b1 < b2 {
            return -1;
        }
        if b1 > b2 {
            return 1;
        }
    }
    
    0
}

unsafe fn compare_ignore_case(idx1: usize, idx2: usize) -> SigmaI32 {
    for i in 0..MAX_LINE_LENGTH {
        let mut b1 = UNIQ_LINES[idx1][i];
        let mut b2 = UNIQ_LINES[idx2][i];
        
        if b1 == 0 && b2 == 0 {
            return 0;
        }
        if b1 == 0 {
            return -1;
        }
        if b2 == 0 {
            return 1;
        }
        
        // Convert to lowercase
        if b1 >= b'A' && b1 <= b'Z' {
            b1 = b1 + 32;
        }
        if b2 >= b'A' && b2 <= b'Z' {
            b2 = b2 + 32;
        }
        
        if b1 < b2 {
            return -1;
        }
        if b1 > b2 {
            return 1;
        }
    }
    
    0
}
