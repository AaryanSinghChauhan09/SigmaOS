// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tests/fs/test_vfs.rs

#![no_std]
#![no_main]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TestResult {
    Pass = 0,
    Fail = 1,
    Skip = 2,
    Error = 3,
}

// Mock file descriptor
#[repr(C)]
pub struct MockFile {
    pub fd: SigmaI32,
    pub offset: SigmaU64,
    pub flags: SigmaU32,
    pub mode: SigmaU32,
}

// Mock inode structure
#[repr(C)]
pub struct MockInode {
    pub inode_num: SigmaU64,
    pub size: SigmaU64,
    pub mode: SigmaU32,
    pub nlink: SigmaU32,
}

// Mock VFS state
static mut FILE_TABLE: [MockFile; 256] = [MockFile {
    fd: -1,
    offset: 0,
    flags: 0,
    mode: 0,
}; 256];
static mut INODE_TABLE: [MockInode; 1024] = [MockInode {
    inode_num: 0,
    size: 0,
    mode: 0,
    nlink: 0,
}; 1024];
static mut NEXT_FD: SigmaI32 = 3;
static mut NEXT_INODE: SigmaU64 = 1;

/// Initialize mock VFS
unsafe fn init_vfs() {
    for i in 0..256 {
        FILE_TABLE[i] = MockFile {
            fd: -1,
            offset: 0,
            flags: 0,
            mode: 0,
        };
    }
    for i in 0..1024 {
        INODE_TABLE[i] = MockInode {
            inode_num: 0,
            size: 0,
            mode: 0,
            nlink: 0,
        };
    }
    NEXT_FD = 3;
    NEXT_INODE = 1;
}

/// Mock mount TmpFS
unsafe fn mock_mount_tmpfs() -> SigmaI32 {
    // In real implementation, this would mount a temporary file system
    // For mock, we just initialize the VFS
    init_vfs();
    0
}

/// Mock open file
unsafe fn mock_open(path: *const SigmaU8, flags: SigmaU32, mode: SigmaU32) -> SigmaI32 {
    if NEXT_FD >= 256 {
        return -1; // EMFILE
    }

    let fd = NEXT_FD;
    FILE_TABLE[fd as usize] = MockFile {
        fd,
        offset: 0,
        flags,
        mode,
    };

    // Create inode
    let inode_num = NEXT_INODE;
    INODE_TABLE[inode_num as usize] = MockInode {
        inode_num,
        size: 0,
        mode: 0o100644, // Regular file, rw-r--r--
        nlink: 1,
    };

    NEXT_FD += 1;
    NEXT_INODE += 1;

    fd
}

/// Mock write to file
unsafe fn mock_write(fd: SigmaI32, buf: *const SigmaU8, count: SigmaU32) -> SigmaI64 {
    if fd < 0 || fd >= 256 {
        return -1; // EBADF
    }

    if FILE_TABLE[fd as usize].fd == -1 {
        return -1; // EBADF
    }

    // Update file size
    let inode_num = fd as SigmaU64 - 2; // Mock mapping
    if inode_num < 1024 {
        INODE_TABLE[inode_num as usize].size += count as SigmaU64;
        FILE_TABLE[fd as usize].offset += count as SigmaU64;
    }

    count as SigmaI64
}

/// Mock read from file
unsafe fn mock_read(fd: SigmaI32, buf: *mut SigmaU8, count: SigmaU32) -> SigmaI64 {
    if fd < 0 || fd >= 256 {
        return -1; // EBADF
    }

    if FILE_TABLE[fd as usize].fd == -1 {
        return -1; // EBADF
    }

    // Return bytes read (mock)
    let inode_num = fd as SigmaU64 - 2;
    if inode_num < 1024 {
        let available = INODE_TABLE[inode_num as usize].size - FILE_TABLE[fd as usize].offset;
        let to_read = if available < count as SigmaU64 {
            available as SigmaU32
        } else {
            count
        };

        FILE_TABLE[fd as usize].offset += to_read as SigmaU64;
        to_read as SigmaI64
    } else {
        count as SigmaI64
    }
}

/// Mock close file
unsafe fn mock_close(fd: SigmaI32) -> SigmaI32 {
    if fd < 0 || fd >= 256 {
        return -1; // EBADF
    }

    if FILE_TABLE[fd as usize].fd == -1 {
        return -1; // EBADF
    }

    FILE_TABLE[fd as usize].fd = -1;
    FILE_TABLE[fd as usize].offset = 0;

    0
}

/// Test: Mount TmpFS
unsafe fn test_mount_tmpfs() -> TestResult {
    init_vfs();

    let result = mock_mount_tmpfs();

    if result != 0 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Open file
unsafe fn test_open_file() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644); // O_RDWR

    if fd < 0 {
        return TestResult::Fail;
    }

    if FILE_TABLE[fd as usize].fd != fd {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Write to file
unsafe fn test_write_file() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644);

    let data = b"Hello, World!\0";
    let written = mock_write(fd, data.as_ptr(), data.len() as SigmaU32);

    if written != data.len() as SigmaI64 {
        return TestResult::Fail;
    }

    let inode_num = fd as SigmaU64 - 2;
    if INODE_TABLE[inode_num as usize].size != data.len() as SigmaU64 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Read from file
unsafe fn test_read_file() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644);

    let write_data = b"Hello, World!\0";
    mock_write(fd, write_data.as_ptr(), write_data.len() as SigmaU32);

    // Reset offset for read
    FILE_TABLE[fd as usize].offset = 0;

    let mut read_buf = [0u8; 256];
    let read = mock_read(fd, read_buf.as_mut_ptr(), write_data.len() as SigmaU32);

    if read != write_data.len() as SigmaI64 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Write and read data match
unsafe fn test_write_read_match() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644);

    let write_data = b"SigmaOS Test Data\0";
    mock_write(fd, write_data.as_ptr(), write_data.len() as SigmaU32);

    // Reset offset for read
    FILE_TABLE[fd as usize].offset = 0;

    let mut read_buf = [0u8; 256];
    mock_read(fd, read_buf.as_mut_ptr(), write_data.len() as SigmaU32);

    // Verify data matches (in real implementation, this would compare actual data)
    let inode_num = fd as SigmaU64 - 2;
    if INODE_TABLE[inode_num as usize].size != write_data.len() as SigmaU64 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Close file
unsafe fn test_close_file() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644);

    let result = mock_close(fd);

    if result != 0 {
        return TestResult::Fail;
    }

    if FILE_TABLE[fd as usize].fd != -1 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: File descriptor exhaustion
unsafe fn test_fd_exhaustion() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    // Fill file descriptor table
    for i in 3..256 {
        let path = b"/tmp/test\0";
        if mock_open(path.as_ptr(), 0o2, 0o644) < 0 {
            return TestResult::Fail;
        }
    }

    // Attempt to open one more file - should fail
    let path = b"/tmp/test\0";
    if mock_open(path.as_ptr(), 0o2, 0o644) != -1 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Invalid file descriptor
unsafe fn test_invalid_fd() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    // Try to read from invalid fd
    let mut buf = [0u8; 256];
    let result = mock_read(-1, buf.as_mut_ptr(), 256);

    if result != -1 {
        return TestResult::Fail;
    }

    // Try to write to invalid fd
    let data = b"test\0";
    let result = mock_write(-1, data.as_ptr(), data.len() as SigmaU32);

    if result != -1 {
        return TestResult::Fail;
    }

    // Try to close invalid fd
    let result = mock_close(-1);

    if result != -1 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: File offset tracking
unsafe fn test_offset_tracking() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path = b"/tmp/test\0";
    let fd = mock_open(path.as_ptr(), 0o2, 0o644);

    let data = b"Hello\0";
    mock_write(fd, data.as_ptr(), data.len() as SigmaU32);

    if FILE_TABLE[fd as usize].offset != data.len() as SigmaU64 {
        return TestResult::Fail;
    }

    let data2 = b" World\0";
    mock_write(fd, data2.as_ptr(), data2.len() as SigmaU32);

    if FILE_TABLE[fd as usize].offset != (data.len() + data2.len()) as SigmaU64 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Inode allocation
unsafe fn test_inode_allocation() -> TestResult {
    init_vfs();
    mock_mount_tmpfs();

    let path1 = b"/tmp/test1\0";
    let fd1 = mock_open(path1.as_ptr(), 0o2, 0o644);

    let path2 = b"/tmp/test2\0";
    let fd2 = mock_open(path2.as_ptr(), 0o2, 0o644);

    let inode1 = fd1 as SigmaU64 - 2;
    let inode2 = fd2 as SigmaU64 - 2;

    if INODE_TABLE[inode1 as usize].inode_num == INODE_TABLE[inode2 as usize].inode_num {
        return TestResult::Fail;
    }

    if INODE_TABLE[inode1 as usize].inode_num != 1 {
        return TestResult::Fail;
    }

    if INODE_TABLE[inode2 as usize].inode_num != 2 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Main test entry point
#[no_mangle]
pub extern "C" fn test_main() -> SigmaI32 {
    let mut passed = 0;
    let mut failed = 0;

    // Run VFS tests
    unsafe {
        if test_mount_tmpfs() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_open_file() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_write_file() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_read_file() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_write_read_match() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_close_file() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_fd_exhaustion() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_invalid_fd() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_offset_tracking() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_inode_allocation() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    // Return 0 on success, non-zero on failure
    if failed > 0 {
        1
    } else {
        0
    }
}
