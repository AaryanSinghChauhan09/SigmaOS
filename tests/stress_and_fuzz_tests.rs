// SigmaOS Modular Test Suite (Unit, Integration, System, Stress, Fuzz & Matrix)
// Validates file I/O, process scheduling, memory allocation, devices, networking, security, and boot sequences.

use sigmaos::filesystem::{VirtualFilesystem, FileType};
use sigmaos::kernel::{Process, ProcessState, Priority, BuddyAllocator};
use sigmaos::network::{DhcpClient, DhcpState, DnsResolver};
use sigmaos::boot::{UefiBootloader, BootType, KernelCmdlineOptions};
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_matrix_file_io() {
    let mut vfs = VirtualFilesystem::new();
    let file_id = vfs.create_file(FileType::Regular, 1000).unwrap();
    let fd = vfs.open_file(file_id, 0).unwrap();

    let input_data = b"SIGMAOS_SOVEREIGN_FILE_IO_TEST_STRING";
    let written = vfs.write_file(fd, input_data).unwrap();
    assert_eq!(written, input_data.len());

    let mut read_buf = [0u8; 64];
    let read_bytes = vfs.read_file(fd, &mut read_buf).unwrap();
    assert_eq!(read_bytes, input_data.len());
}

#[test]
fn test_matrix_memory_stress_allocation() {
    let mut buddy = BuddyAllocator::new(1024 * 1024); // 1MB pool
    let mut ptrs = Vec::new();

    // Allocate 100 blocks
    for _ in 0..100 {
        if let Ok(ptr) = buddy.allocate(4096) {
            ptrs.push(ptr);
        }
    }

    // Free 100 blocks
    for ptr in ptrs {
        buddy.deallocate(ptr, 4096);
    }

    assert_eq!(buddy.allocated_bytes, 0);
}

#[test]
fn test_matrix_syscall_fuzzing() {
    let mut vfs = VirtualFilesystem::new();

    // Invalid FD fuzzing
    assert!(vfs.read_file(999999, &mut [0u8; 16]).is_err());
    assert!(vfs.write_file(999999, b"invalid_fd_fuzz").is_err());
    assert!(vfs.close_file(999999).is_err());

    // Invalid path unveil fuzzing
    let mut unveil = UnveilManager::new();
    unveil.unveil("/var", "r").unwrap();
    assert!(unveil.validate_path("/etc/shadow", UnveilPermission::Read).is_err());
    assert!(unveil.validate_path("\0\0\0/invalid_fuzz", UnveilPermission::Write).is_err());
}

#[test]
fn test_matrix_boot_sequence_and_cmdline() {
    let opts = KernelCmdlineOptions::parse("console=ttyS0 quiet root=/dev/nvme0n1p2 panic=1");
    assert_eq!(opts.console, "ttyS0");
    assert!(opts.quiet);
    assert_eq!(opts.root_device, "/dev/nvme0n1p2");
    assert_eq!(opts.custom_params.get("panic").unwrap(), "1");

    let mut bootloader = UefiBootloader::new(BootType::UefiGpt);
    assert!(bootloader.parse_gpt_header(&[0u8; 512]).is_ok());
    assert!(bootloader.load_kernel_elf(b"\x7FELF_STRESS_BOOT_TEST").is_ok());
}

#[test]
fn test_matrix_networking_dhcp_dns() {
    let mut dhcp = DhcpClient::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    assert_eq!(dhcp.send_discover(), DhcpState::Discover);
    assert_eq!(dhcp.process_offer("10.0.0.150", "10.0.0.1"), DhcpState::Bound);

    let dns = DnsResolver::new();
    assert_eq!(dns.resolve_a_record("dns.google").unwrap(), "8.8.8.8");
}
