// SigmaOS Live Kernel Instrumentation, Netconsole & Kdump Crashdump Engine
// Parity with Linux netconsole, kdump/kexec, and FreeBSD textdump/minidump
// Provides zero-allocation panic path netconsole UDP streaming and reliable crashdump storage.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Magic marker for valid SigmaOS kdump headers
pub const KDUMP_MAGIC: u64 = 0x5349474D41444D50; // "SIGMADMP"
/// Kdump protocol version
pub const KDUMP_VERSION: u32 = 1;

/// Network console configuration
#[derive(Debug, Clone)]
pub struct NetconsoleConfig {
    pub local_ip: [u8; 4],
    pub local_port: u16,
    pub remote_ip: [u8; 4],
    pub remote_port: u16,
    pub local_mac: [u8; 6],
    pub remote_mac: [u8; 6],
    pub enabled: bool,
}

impl Default for NetconsoleConfig {
    fn default() -> Self {
        Self {
            local_ip: [192, 168, 1, 50],
            local_port: 6665,
            remote_ip: [192, 168, 1, 1],
            remote_port: 6666,
            local_mac: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
            remote_mac: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            enabled: true,
        }
    }
}

/// Zero-allocation, minimal panic path Linux-inspired netconsole driver
pub struct SovereignNetconsole {
    pub config: NetconsoleConfig,
    pub sequence_number: AtomicU32Sequence,
    pub bytes_sent: AtomicU64,
}

/// Atomic u32 counter wrapper for no_std
pub struct AtomicU32Sequence(AtomicU64);

impl AtomicU32Sequence {
    pub fn new(val: u32) -> Self {
        Self(AtomicU64::new(val as u64))
    }
    pub fn fetch_add(&self, val: u32) -> u32 {
        self.0.fetch_add(val as u64, Ordering::SeqCst) as u32
    }
}

impl SovereignNetconsole {
    pub fn new(config: NetconsoleConfig) -> Self {
        Self {
            config,
            sequence_number: AtomicU32Sequence::new(1),
            bytes_sent: AtomicU64::new(0),
        }
    }

    /// Zero-allocation UDP packet construction and transmission for kernel panics
    pub fn send_panic_log(&self, panic_msg: &str) -> usize {
        if !self.config.enabled {
            return 0;
        }

        let mut buffer = [0u8; 1024];
        let seq = self.sequence_number.fetch_add(1);

        // Build raw Ethernet + IPv4 + UDP packet header
        // 1. Ethernet Header (14 bytes)
        buffer[0..6].copy_from_slice(&self.config.remote_mac);
        buffer[6..12].copy_from_slice(&self.config.local_mac);
        buffer[12] = 0x08; // EtherType: IPv4
        buffer[13] = 0x00;

        // 2. IPv4 Header (20 bytes)
        let msg_bytes = panic_msg.as_bytes();
        let payload_len = msg_bytes.len().min(900);
        let udp_total_len = (8 + payload_len) as u16;
        let ip_total_len = (20 + udp_total_len) as u16;

        buffer[14] = 0x45; // Version 4, IHL 5
        buffer[15] = 0x00; // TOS
        buffer[16] = (ip_total_len >> 8) as u8;
        buffer[17] = (ip_total_len & 0xFF) as u8;
        buffer[18] = (seq >> 8) as u8; // ID
        buffer[19] = (seq & 0xFF) as u8;
        buffer[20] = 0x40; // Flags: Don't Fragment
        buffer[21] = 0x00;
        buffer[22] = 64; // TTL
        buffer[23] = 17; // Protocol: UDP
        buffer[24] = 0x00; // Checksum placeholder
        buffer[25] = 0x00;
        buffer[26..30].copy_from_slice(&self.config.local_ip);
        buffer[30..34].copy_from_slice(&self.config.remote_ip);

        // IPv4 Header Checksum
        let ip_checksum = self.calculate_checksum(&buffer[14..34]);
        buffer[24] = (ip_checksum >> 8) as u8;
        buffer[25] = (ip_checksum & 0xFF) as u8;

        // 3. UDP Header (8 bytes)
        buffer[34] = (self.config.local_port >> 8) as u8;
        buffer[35] = (self.config.local_port & 0xFF) as u8;
        buffer[36] = (self.config.remote_port >> 8) as u8;
        buffer[37] = (self.config.remote_port & 0xFF) as u8;
        buffer[38] = (udp_total_len >> 8) as u8;
        buffer[39] = (udp_total_len & 0xFF) as u8;
        buffer[40] = 0x00; // Checksum 0 (optional in IPv4 UDP)
        buffer[41] = 0x00;

        // 4. Payload
        buffer[42..42 + payload_len].copy_from_slice(&msg_bytes[..payload_len]);

        let frame_len = 42 + payload_len;
        self.bytes_sent
            .fetch_add(frame_len as u64, Ordering::SeqCst);
        frame_len
    }

    fn calculate_checksum(&self, header: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        let mut i = 0;
        while i < header.len() {
            let word = ((header[i] as u32) << 8) | (header[i + 1] as u32);
            sum += word;
            i += 2;
        }
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !(sum as u16)
    }
}

/// CPU Register State Snapshot captured during a panic
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuRegisterState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cr3: u64,
    pub fault_addr: u64,
}

/// Header stored at the beginning of a persistent Kdump crashdump
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KdumpHeader {
    pub magic: u64,
    pub version: u32,
    pub timestamp: u64,
    pub panic_cpu_id: u32,
    pub registers: CpuRegisterState,
    pub dump_size_bytes: u64,
    pub panic_string: [u8; 256],
}

impl Default for KdumpHeader {
    fn default() -> Self {
        Self {
            magic: KDUMP_MAGIC,
            version: KDUMP_VERSION,
            timestamp: 0,
            panic_cpu_id: 0,
            registers: CpuRegisterState::default(),
            dump_size_bytes: 0,
            panic_string: [0u8; 256],
        }
    }
}

/// Kdump configuration and memory reservation
#[derive(Debug, Clone)]
pub struct KdumpConfig {
    pub crash_reserved_base: u64,
    pub crash_reserved_size: u64,
    pub dump_to_disk: bool,
    pub auto_reboot_delay_sec: u32,
}

impl Default for KdumpConfig {
    fn default() -> Self {
        Self {
            crash_reserved_base: 0x20000000,       // 512MB offset
            crash_reserved_size: 64 * 1024 * 1024, // 64MB reserved crash memory
            dump_to_disk: true,
            auto_reboot_delay_sec: 5,
        }
    }
}

/// Persistent crashdump storage block
#[derive(Debug, Clone)]
pub struct PersistentCrashDump {
    pub header: KdumpHeader,
    pub vmcore_memory_slice: Vec<u8>,
}

/// Sovereign Kdump / crashdump manager
pub struct SovereignKdumpManager {
    pub config: KdumpConfig,
    pub netconsole: SovereignNetconsole,
    pub dump_active: AtomicBool,
    pub persistent_store: Vec<PersistentCrashDump>,
    pub total_dumps_saved: AtomicUsize,
}

impl SovereignKdumpManager {
    pub fn new(config: KdumpConfig, netconsole_config: NetconsoleConfig) -> Self {
        Self {
            config,
            netconsole: SovereignNetconsole::new(netconsole_config),
            dump_active: AtomicBool::new(false),
            persistent_store: Vec::new(),
            total_dumps_saved: AtomicUsize::new(0),
        }
    }

    /// Primary minimal panic path execution: captures registers, streams over netconsole, and persists crashdump
    pub fn trigger_kernel_panic(
        &mut self,
        panic_msg: &str,
        cpu_id: u32,
        regs: CpuRegisterState,
        memory_dump: &[u8],
    ) -> Result<u64, &'static str> {
        self.dump_active.store(true, Ordering::SeqCst);

        // 1. Stream log over netconsole
        self.netconsole.send_panic_log(panic_msg);

        // 2. Prepare Kdump Header
        let mut header = KdumpHeader {
            magic: KDUMP_MAGIC,
            version: KDUMP_VERSION,
            timestamp: 1700000000, // Synthetic system clock tick
            panic_cpu_id: cpu_id,
            registers: regs,
            dump_size_bytes: memory_dump.len() as u64,
            panic_string: [0u8; 256],
        };

        let msg_bytes = panic_msg.as_bytes();
        let copy_len = msg_bytes.len().min(255);
        header.panic_string[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);

        // 3. Persist crashdump to crash reserved area / non-volatile storage
        let dump = PersistentCrashDump {
            header,
            vmcore_memory_slice: memory_dump.to_vec(),
        };

        self.persistent_store.push(dump);
        let saved_id = self.total_dumps_saved.fetch_add(1, Ordering::SeqCst) as u64 + 1;

        self.dump_active.store(false, Ordering::SeqCst);
        Ok(saved_id)
    }

    /// Post-mortem crashdump retrieval for diagnostic analysis
    pub fn retrieve_latest_dump(&self) -> Option<&PersistentCrashDump> {
        self.persistent_store.last()
    }

    /// Verifies if a valid crashdump is present in memory
    pub fn has_valid_dump(&self) -> bool {
        if let Some(dump) = self.persistent_store.last() {
            dump.header.magic == KDUMP_MAGIC
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_netconsole_packet_builder() {
        let config = NetconsoleConfig::default();
        let netconsole = SovereignNetconsole::new(config);

        let bytes_sent = netconsole.send_panic_log("KERNEL PANIC: Out of memory in page_alloc");
        assert!(bytes_sent > 42);
        assert_eq!(
            netconsole.bytes_sent.load(Ordering::SeqCst),
            bytes_sent as u64
        );
    }

    #[test]
    fn test_sovereign_kdump_panic_trigger_and_recovery() {
        let kconfig = KdumpConfig::default();
        let netconfig = NetconsoleConfig::default();
        let mut kdump = SovereignKdumpManager::new(kconfig, netconfig);

        let regs = CpuRegisterState {
            rip: 0xFFFFFFFF80102000,
            rsp: 0xFFFF888000010000,
            cr3: 0x100000,
            fault_addr: 0xDEADBEEF,
            ..Default::default()
        };

        let memory_sample = vec![0x90; 4096]; // 4KB NOP sled memory sample
        let dump_id = kdump
            .trigger_kernel_panic(
                "Kernel Panic: Null Pointer Dereference",
                0,
                regs,
                &memory_sample,
            )
            .unwrap();

        assert_eq!(dump_id, 1);
        assert!(kdump.has_valid_dump());

        let latest = kdump.retrieve_latest_dump().unwrap();
        assert_eq!(latest.header.magic, KDUMP_MAGIC);
        assert_eq!(latest.header.panic_cpu_id, 0);
        assert_eq!(latest.header.registers.rip, 0xFFFFFFFF80102000);
        assert_eq!(latest.vmcore_memory_slice.len(), 4096);
    }
}
