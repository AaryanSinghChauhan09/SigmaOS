//! SigmaOS Sort Compatibility
//! Sort lines of text (sort command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Sort order
#[repr(C)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Sort mode
#[repr(C)]
pub enum SortMode {
    Numeric,
    Alphabetic,
    Version,
    Random,
}

/// Sort options
#[repr(C)]
pub struct SortOptions {
    pub order: SortOrder,
    pub mode: SortMode,
    pub unique: SigmaBool,
    pub reverse: SigmaBool,
    pub ignore_case: SigmaBool,
    pub field: SigmaU32,
}

/// Sort state
const MAX_LINES: usize = 10000;
const MAX_LINE_LENGTH: usize = 1024;

static mut SORT_LINES: [[u8; MAX_LINE_LENGTH]; MAX_LINES] = [[0; MAX_LINE_LENGTH]; MAX_LINES];
static mut SORT_LINE_COUNT: SigmaU32 = 0;
static mut SORT_INITIALIZED: SigmaBool = false;

/// Initialize sort
#[no_mangle]
pub unsafe extern "C" fn sort_init() -> SigmaI32 {
    SORT_INITIALIZED = true;
    SORT_LINE_COUNT = 0;
    
    0 // Success
}

/// Add line to sort
#[no_mangle]
pub unsafe extern "C" fn sort_add_line(line: *const u8) -> SigmaI32 {
    if !SORT_INITIALIZED || line.isnull() || SORT_LINE_COUNT >= MAX_LINES as SigmaU32 {
        return -1;
    }
    
    for i in 0..MAX_LINE_LENGTH - 1 {
        let byte = *line.add(i);
        if byte == 0 || byte == b'\n' { break; }
        SORT_LINES[SORT_LINE_COUNT as usize][i] = byte;
    }
    
    SORT_LINE_COUNT += 1;
    
    0 // Success
}

/// Sort lines
#[no_mangle]
pub unsafe extern "C" fn sort_execute(options: SortOptions) -> SigmaI32 {
    if !SORT_INITIALIZED {
        return -1;
    }
    
    // Simple bubble sort (in a real implementation, use a more efficient algorithm)
    for i in 0..SORT_LINE_COUNT as usize {
        for j in 0..(SORT_LINE_COUNT as usize - i - 1) {
            let mut should_swap = false;
            
            match options.mode {
                SortMode::Alphabetic => {
                    if options.ignore_case {
                        should_swap = compare_ignore_case(j, j + 1) > 0;
                    } else {
                        should_swap = compare_lines(j, j + 1) > 0;
                    }
                }
                SortMode::Numeric => {
                    should_swap = compare_numeric(j, j + 1) > 0;
                }
                _ => {
                    should_swap = compare_lines(j, j + 1) > 0;
                }
            }
            
            if options.order == SortOrder::Descending {
                should_swap = !should_swap;
            }
            
            if should_swap {
                for k in 0..MAX_LINE_LENGTH {
                    let temp = SORT_LINES[j][k];
                    SORT_LINES[j][k] = SORT_LINES[j + 1][k];
                    SORT_LINES[j + 1][k] = temp;
                }
            }
        }
    }
    
    if options.unique {
        // Remove duplicates
        let mut write_idx = 1;
        for i in 1..SORT_LINE_COUNT as usize {
            if compare_lines(write_idx - 1, i) != 0 {
                if write_idx != i {
                    for k in 0..MAX_LINE_LENGTH {
                        SORT_LINES[write_idx][k] = SORT_LINES[i][k];
                    }
                }
                write_idx += 1;
            }
        }
        SORT_LINE_COUNT = write_idx as SigmaU32;
    }
    
    0 // Success
}

/// Get sorted lines
#[no_mangle]
pub unsafe extern "C" fn sort_get_lines(
    output: *mut u8,
    max_output: SigmaU32,
) -> SigmaU32 {
    if !SORT_INITIALIZED || output.isnull() {
        return 0;
    }
    
    let mut output_idx = 0;
    
    for i in 0..SORT_LINE_COUNT as usize {
        for j in 0..MAX_LINE_LENGTH {
            if output_idx >= max_output as usize {
                return output_idx;
            }
            
            let byte = SORT_LINES[i][j];
            if byte == 0 { break; }
            
            *output.add(output_idx) = byte;
            output_idx += 1;
        }
        
        if output_idx < max_output as usize {
            *output.add(output_idx) = b'\n';
            output_idx += 1;
        }
    }
    
    output_idx
}

/// Clear lines
#[no_mangle]
pub unsafe extern "C" fn sort_clear() -> SigmaI32 {
    if !SORT_INITIALIZED {
        return -1;
    }
    
    SORT_LINE_COUNT = 0;
    
    0 // Success
}

/// Get line count
#[no_mangle]
pub unsafe extern "C" fn sort_get_line_count() -> SigmaU32 {
    SORT_LINE_COUNT
}

unsafe fn compare_lines(idx1: usize, idx2: usize) -> SigmaI32 {
    for i in 0..MAX_LINE_LENGTH {
        let b1 = SORT_LINES[idx1][i];
        let b2 = SORT_LINES[idx2][i];
        
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
        let mut b1 = SORT_LINES[idx1][i];
        let mut b2 = SORT_LINES[idx2][i];
        
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

unsafe fn compare_numeric(idx1: usize, idx2: usize) -> SigmaI32 {
    let mut n1: SigmaI32 = 0;
    let mut n2: SigmaI32 = 0;
    
    // Parse first number from line 1
    for i in 0..MAX_LINE_LENGTH {
        let b = SORT_LINES[idx1][i];
        if b == 0 { break; }
        if b >= b'0' && b <= b'9' {
            n1 = n1 * 10 + (b - b'0') as SigmaI32;
        } else if n1 > 0 {
            break;
        }
    }
    
    // Parse first number from line 2
    for i in 0..MAX_LINE_LENGTH {
        let b = SORT_LINES[idx2][i];
        if b == 0 { break; }
        if b >= b'0' && b <= b'9' {
            n2 = n2 * 10 + (b - b'0') as SigmaI32;
        } else if n2 > 0 {
            break;
        }
    }
    
    if n1 < n2 {
        -1
    } else if n1 > n2 {
        1
    } else {
        0
    }
}
