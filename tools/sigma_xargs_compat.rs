//! SigmaOS Xargs Compatibility
//! Build and execute command lines from stdin (xargs command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Xargs options
#[repr(C)]
pub struct XargsOptions {
    pub max_args: SigmaU32,
    pub max_lines: SigmaU32,
    pub no_run_if_empty: SigmaBool,
    pub verbose: SigmaBool,
    pub delimiter: [u8; 16],
}

/// Xargs state
const MAX_ARGS: usize = 1000;
const MAX_ARG_LENGTH: usize = 512;

static mut XARGS_ARGS: [[u8; MAX_ARG_LENGTH]; MAX_ARGS] = [[0; MAX_ARG_LENGTH]; MAX_ARGS];
static mut XARGS_ARG_COUNT: SigmaU32 = 0;
static mut XARGS_INITIALIZED: SigmaBool = false;

/// Initialize xargs
#[no_mangle]
pub unsafe extern "C" fn xargs_init() -> SigmaI32 {
    XARGS_INITIALIZED = true;
    XARGS_ARG_COUNT = 0;
    
    0 // Success
}

/// Add argument
#[no_mangle]
pub unsafe extern "C" fn xargs_add_arg(arg: *const u8) -> SigmaI32 {
    if !XARGS_INITIALIZED || arg.isnull() || XARGS_ARG_COUNT >= MAX_ARGS as SigmaU32 {
        return -1;
    }
    
    for i in 0..MAX_ARG_LENGTH - 1 {
        let byte = *arg.add(i);
        if byte == 0 || byte == b'\n' || byte == b' ' { break; }
        XARGS_ARGS[XARGS_ARG_COUNT as usize][i] = byte;
    }
    
    XARGS_ARG_COUNT += 1;
    
    0 // Success
}

/// Parse input into arguments
#[no_mangle]
pub unsafe extern "C" fn xargs_parse_input(
    input: *const u8,
    delimiter: *const u8,
) -> SigmaI32 {
    if !XARGS_INITIALIZED || input.isnull() || delimiter.isnull() {
        return -1;
    }
    
    let mut arg_start = 0;
    let mut i = 0;
    
    loop {
        let byte = *input.add(i);
        if byte == 0 { break; }
        
        // Check for delimiter
        let mut is_delimiter = false;
        let mut j = 0;
        while j < 16 {
            let delim_byte = *delimiter.add(j);
            if delim_byte == 0 { break; }
            if *input.add(i + j) != delim_byte {
                is_delimiter = false;
                break;
            }
            is_delimiter = true;
            j += 1;
        }
        
        if is_delimiter && j > 0 {
            // Add argument
            if XARGS_ARG_COUNT < MAX_ARGS as SigmaU32 {
                let mut len = 0;
                for k in arg_start..i {
                    if len < MAX_ARG_LENGTH - 1 {
                        XARGS_ARGS[XARGS_ARG_COUNT as usize][len] = *input.add(k);
                        len += 1;
                    }
                }
                XARGS_ARG_COUNT += 1;
            }
            
            arg_start = i + j;
            i += j;
        } else {
            i += 1;
        }
    }
    
    // Add last argument
    if arg_start < i && XARGS_ARG_COUNT < MAX_ARGS as SigmaU32 {
        let mut len = 0;
        for k in arg_start..i {
            if len < MAX_ARG_LENGTH - 1 {
                XARGS_ARGS[XARGS_ARG_COUNT as usize][len] = *input.add(k);
                len += 1;
            }
        }
        XARGS_ARG_COUNT += 1;
    }
    
    0 // Success
}

/// Build command line
#[no_mangle]
pub unsafe extern "C" fn xargs_build_command(
    command: *const u8,
    output: *mut u8,
    max_output: SigmaU32,
    options: XargsOptions,
) -> SigmaI32 {
    if !XARGS_INITIALIZED || command.isnull() || output.isnull() {
        return -1;
    }
    
    let mut output_idx = 0;
    
    // Add command
    for i in 0..511 {
        let byte = *command.add(i);
        if byte == 0 { break; }
        if output_idx >= max_output as usize {
            return -1;
        }
        *output.add(output_idx) = byte;
        output_idx += 1;
    }
    
    if output_idx < max_output as usize {
        *output.add(output_idx) = b' ';
        output_idx += 1;
    }
    
    // Add arguments
    let mut args_added = 0;
    for i in 0..XARGS_ARG_COUNT as usize {
        if options.max_args > 0 && args_added >= options.max_args as usize {
            break;
        }
        
        for j in 0..MAX_ARG_LENGTH {
            if output_idx >= max_output as usize {
                return -1;
            }
            
            let byte = XARGS_ARGS[i][j];
            if byte == 0 { break; }
            
            *output.add(output_idx) = byte;
            output_idx += 1;
        }
        
        if output_idx < max_output as usize {
            *output.add(output_idx) = b' ';
            output_idx += 1;
        }
        
        args_added += 1;
    }
    
    0 // Success
}

/// Get arguments
#[no_mangle]
pub unsafe extern "C" fn xargs_get_args(
    args: *mut *const u8,
    max_count: SigmaU32,
) -> SigmaU32 {
    if !XARGS_INITIALIZED || args.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..XARGS_ARG_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *args.add(count) = XARGS_ARGS[i].as_ptr();
        count += 1;
    }
    
    count
}

/// Get arg count
#[no_mangle]
pub unsafe extern "C" fn xargs_get_arg_count() -> SigmaU32 {
    XARGS_ARG_COUNT
}

/// Clear arguments
#[no_mangle]
pub unsafe extern "C" fn xargs_clear() -> SigmaI32 {
    if !XARGS_INITIALIZED {
        return -1;
    }
    
    XARGS_ARG_COUNT = 0;
    
    0 // Success
}
