// SPDX-License-Identifier: Apache-2.0
#![allow(unused_variables)]
//! SigmaOS Disk Usage Compatibility
//! Directory space usage (du command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Directory entry
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirEntry {
    pub path: [u8; 512],
    pub apparent_size: SigmaU64,
    pub block_size: SigmaU64,
    pub depth: SigmaU32,
    pub mtime: SigmaU64,
    pub is_directory: SigmaBool,
}

/// Disk usage state
const MAX_DIR_ENTRIES: usize = 10000;

static mut DIR_ENTRIES: [DirEntry; MAX_DIR_ENTRIES] = [DirEntry {
    path: [0; 512],
    apparent_size: 0,
    block_size: 0,
    depth: 0,
    mtime: 0,
    is_directory: false,
}; MAX_DIR_ENTRIES];

static mut DIR_ENTRY_COUNT: SigmaU32 = 0;
static mut DU_INITIALIZED: SigmaBool = false;

// Linux-inspired Advanced Settings
static mut DU_MAX_DEPTH: SigmaU32 = 999;
static mut DU_APPARENT_SIZE: SigmaBool = false;
static mut DU_EXCLUDE_PATTERN: [u8; 128] = [0; 128];
static mut DU_SUMMARIZE: SigmaBool = false;

/// Initialize du
#[no_mangle]
pub unsafe extern "C" fn du_init() -> SigmaI32 {
    DU_INITIALIZED = true;
    DIR_ENTRY_COUNT = 0;
    DU_MAX_DEPTH = 999;
    DU_APPARENT_SIZE = false;
    DU_SUMMARIZE = false;
    for i in 0..128 {
        DU_EXCLUDE_PATTERN[i] = 0;
    }
    0 // Success
}

/// Set max-depth recursion limit (matches -d, --max-depth in Linux du)
#[no_mangle]
pub unsafe extern "C" fn du_set_max_depth(depth: SigmaU32) {
    DU_MAX_DEPTH = depth;
}

/// Toggle apparent size vs block size tracking (matches --apparent-size in Linux du)
#[no_mangle]
pub unsafe extern "C" fn du_set_apparent_size(enabled: SigmaBool) {
    DU_APPARENT_SIZE = enabled;
}

/// Toggle summarize flag (matches -s, --summarize in Linux du)
#[no_mangle]
pub unsafe extern "C" fn du_set_summarize(enabled: SigmaBool) {
    DU_SUMMARIZE = enabled;
}

/// Set string pattern to exclude paths (matches --exclude in Linux du)
#[no_mangle]
pub unsafe extern "C" fn du_set_exclude_pattern(pattern: *const u8) -> SigmaI32 {
    if pattern.is_null() {
        return -1;
    }
    for i in 0..127 {
        let byte = *pattern.add(i);
        DU_EXCLUDE_PATTERN[i] = byte;
        if byte == 0 { break; }
    }
    0
}

/// Helper: check if path contains exclusion pattern substring
unsafe fn is_excluded(path: &[u8]) -> bool {
    if DU_EXCLUDE_PATTERN[0] == 0 {
        return false;
    }
    
    // Find pattern length
    let mut pattern_len = 0;
    while pattern_len < 128 && DU_EXCLUDE_PATTERN[pattern_len] != 0 {
        pattern_len += 1;
    }

    if pattern_len == 0 {
        return false;
    }

    // Find path length
    let mut path_len = 0;
    while path_len < path.len() && path[path_len] != 0 {
        path_len += 1;
    }

    if path_len < pattern_len {
        return false;
    }

    // Substring matching
    for i in 0..=(path_len - pattern_len) {
        let mut found = true;
        for j in 0..pattern_len {
            if path[i + j] != DU_EXCLUDE_PATTERN[j] {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }

    false
}

/// Advanced scan entry with Linux parameters (max depth, block sizes, and exclusions)
#[no_mangle]
pub unsafe extern "C" fn du_scan_advanced(
    path: *const u8,
    size_bytes: SigmaU64,
    depth: SigmaU32,
    mtime: SigmaU64,
    is_dir: SigmaBool,
) -> SigmaI32 {
    if !DU_INITIALIZED || path.is_null() {
        return -1;
    }

    // 1. Enforce depth constraint (matches --max-depth)
    if depth > DU_MAX_DEPTH {
        return 0; // Skip silently without error
    }

    // 2. Map path to array
    let mut path_array = [0u8; 512];
    for i in 0..511 {
        let byte = *path.add(i);
        if byte == 0 { break; }
        path_array[i] = byte;
    }

    // 3. Enforce path exclusion (matches --exclude)
    if is_excluded(&path_array) {
        return 0; // Exclude silently
    }

    // 4. Summarize logic (only display depth 0/total elements if summarize is true)
    if DU_SUMMARIZE && depth > 0 {
        return 0;
    }

    // 5. Apparent vs block-allocated size calculations (aligned to 4KB blocks)
    let apparent_size = size_bytes;
    let block_size = if DU_APPARENT_SIZE {
        apparent_size
    } else {
        // Round up to nearest 4KB block size
        ((apparent_size + 4095) / 4096) * 4096
    };

    let entry = DirEntry {
        path: path_array,
        apparent_size,
        block_size,
        depth,
        mtime,
        is_directory: is_dir,
    };

    if DIR_ENTRY_COUNT as usize >= MAX_DIR_ENTRIES {
        return -2; // Out of capacity
    }

    DIR_ENTRIES[DIR_ENTRY_COUNT as usize] = entry;
    DIR_ENTRY_COUNT += 1;

    0 // Success
}

/// Simple legacy scanner wrapper
#[no_mangle]
pub unsafe extern "C" fn du_scan(path: *const u8) -> SigmaI32 {
    du_scan_advanced(path, 1024, 0, 1625100000, true)
}

/// Get directory entries
#[no_mangle]
pub unsafe extern "C" fn du_list(entries: *mut DirEntry, max_count: SigmaU32) -> SigmaU32 {
    if !DU_INITIALIZED || entries.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..DIR_ENTRY_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *entries.add(count) = DIR_ENTRIES[i];
        count += 1;
    }
    
    count as SigmaU32
}

/// Get total size of path
#[no_mangle]
pub unsafe extern "C" fn du_get_size(path: *const u8, size: *mut SigmaU64) -> SigmaI32 {
    if !DU_INITIALIZED || path.is_null() || size.is_null() {
        return -1;
    }
    
    for i in 0..DIR_ENTRY_COUNT as usize {
        let entry = &DIR_ENTRIES[i];
        
        let mut matches = true;
        for j in 0..512 {
            if entry.path[j] != *path.add(j) {
                if entry.path[j] == 0 && *path.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if entry.path[j] == 0 {
                break;
            }
        }
        
        if matches {
            // Return either apparent or block size depending on configuration
            *size = if DU_APPARENT_SIZE { entry.apparent_size } else { entry.block_size };
            return 0;
        }
    }
    
    -2 // Path not found
}

/// Get entry count
#[no_mangle]
pub unsafe extern "C" fn du_get_entry_count() -> SigmaU32 {
    DIR_ENTRY_COUNT
}

/// Clear entries
#[no_mangle]
pub unsafe extern "C" fn du_clear() -> SigmaI32 {
    if !DU_INITIALIZED {
        return -1;
    }
    
    DIR_ENTRY_COUNT = 0;
    
    0 // Success
}

/// Sort by size (descending)
#[no_mangle]
pub unsafe extern "C" fn du_sort_by_size() -> SigmaI32 {
    if !DU_INITIALIZED {
        return -1;
    }
    
    // Simple bubble sort using whichever size metric is currently configured
    for i in 0..DIR_ENTRY_COUNT as usize {
        for j in 0..DIR_ENTRY_COUNT as usize - i - 1 {
            let size_j = if DU_APPARENT_SIZE { DIR_ENTRIES[j].apparent_size } else { DIR_ENTRIES[j].block_size };
            let size_jp1 = if DU_APPARENT_SIZE { DIR_ENTRIES[j + 1].apparent_size } else { DIR_ENTRIES[j + 1].block_size };

            if size_j < size_jp1 {
                let temp = DIR_ENTRIES[j];
                DIR_ENTRIES[j] = DIR_ENTRIES[j + 1];
                DIR_ENTRIES[j + 1] = temp;
            }
        }
    }
    
    0 // Success
}

/// Helper: safely write unsigned integer to raw buffer
unsafe fn write_uint_to_buf(mut val: u64, buf: *mut u8, buf_len: usize) -> usize {
    if buf_len == 0 {
        return 0;
    }
    if val == 0 {
        *buf = b'0';
        return 1;
    }

    let mut temp_digits = [0u8; 24];
    let mut digit_count = 0;
    while val > 0 {
        temp_digits[digit_count] = (val % 10) as u8 + b'0';
        val /= 10;
        digit_count += 1;
    }

    let mut written = 0;
    for i in (0..digit_count).rev() {
        if written >= buf_len {
            break;
        }
        *buf.add(written) = temp_digits[i];
        written += 1;
    }
    written
}

/// Linux-inspired human-readable file size formatter (matches -h, --human-readable in Linux du)
/// Converts byte values into scaled format (e.g. 512 -> "512B", 1024 -> "1.0K", 1048576 -> "1.0M")
#[no_mangle]
pub unsafe extern "C" fn du_format_human_readable(size: SigmaU64, buf: *mut u8, buf_len: usize) -> SigmaI32 {
    if buf.is_null() || buf_len < 8 {
        return -1;
    }

    let units = [b'B', b'K', b'M', b'G', b'T'];
    let mut val = size;
    let mut unit_idx = 0;
    let mut rem = 0;

    while val >= 1024 && unit_idx < units.len() - 1 {
        rem = val % 1024;
        val /= 1024;
        unit_idx += 1;
    }

    let rem_dec = (rem * 10) / 1024;
    let mut written = 0;

    if unit_idx == 0 {
        // Suffix bytes directly e.g. "512B"
        written = write_uint_to_buf(val, buf, buf_len);
    } else {
        // Decimal scaled suffix e.g. "1.2M"
        written = write_uint_to_buf(val, buf, buf_len);
        if written < buf_len {
            *buf.add(written) = b'.';
            written += 1;
        }
        if written < buf_len {
            *buf.add(written) = b'0' + rem_dec as u8;
            written += 1;
        }
    }

    if written < buf_len {
        *buf.add(written) = units[unit_idx];
        written += 1;
    }

    if written < buf_len {
        *buf.add(written) = 0; // Null terminator
    }

    written as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_du_apparent_vs_block_size() {
        unsafe {
            du_init();

            // Set block size tracking (default)
            du_set_apparent_size(false);
            let path1 = b"/home/user/file.txt\0";
            du_scan_advanced(path1.as_ptr(), 10, 0, 0, false);

            let mut size = 0;
            du_get_size(path1.as_ptr(), &mut size);
            // 10 bytes apparent size should align to 4096 (4KB block)
            assert_eq!(size, 4096);

            // Set apparent size tracking
            du_set_apparent_size(true);
            let mut app_size = 0;
            du_get_size(path1.as_ptr(), &mut app_size);
            // Should be the exact apparent size (10 bytes)
            assert_eq!(app_size, 10);
        }
    }

    #[test]
    fn test_du_max_depth_filtration() {
        unsafe {
            du_init();
            du_set_max_depth(1);

            let path_depth0 = b"/var\0";
            let path_depth1 = b"/var/log\0";
            let path_depth2 = b"/var/log/nginx\0";

            du_scan_advanced(path_depth0.as_ptr(), 100, 0, 0, true);
            du_scan_advanced(path_depth1.as_ptr(), 200, 1, 0, true);
            du_scan_advanced(path_depth2.as_ptr(), 300, 2, 0, true); // Depth 2 > Max Depth 1

            assert_eq!(du_get_entry_count(), 2); // Only depth 0 and 1 registered
        }
    }

    #[test]
    fn test_du_exclusion_patterns() {
        unsafe {
            du_init();
            let pattern = b"tmp\0";
            du_set_exclude_pattern(pattern.as_ptr());

            let path_normal = b"/usr/bin\0";
            let path_excluded = b"/usr/tmp/cache\0";

            du_scan_advanced(path_normal.as_ptr(), 100, 0, 0, false);
            du_scan_advanced(path_excluded.as_ptr(), 100, 0, 0, false); // Contains "tmp" -> Excluded

            assert_eq!(du_get_entry_count(), 1);
        }
    }

    #[test]
    fn test_du_human_readable_formatting() {
        unsafe {
            let mut buf = [0u8; 32];

            // Test Bytes
            du_format_human_readable(512, buf.as_mut_ptr(), buf.len());
            assert_eq!(core::str::from_utf8(&buf[..4]).unwrap(), "512B");

            // Test Kilobytes
            du_format_human_readable(1024, buf.as_mut_ptr(), buf.len());
            assert_eq!(core::str::from_utf8(&buf[..4]).unwrap(), "1.0K");

            // Test Megabytes
            du_format_human_readable(1048576, buf.as_mut_ptr(), buf.len());
            assert_eq!(core::str::from_utf8(&buf[..4]).unwrap(), "1.0M");
        }
    }
}
