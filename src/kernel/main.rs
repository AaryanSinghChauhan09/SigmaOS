// SigmaOS Kernel Main Entry Point
#![no_std]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;
use alloc::string::ToString;

use core::sync::atomic::{AtomicUsize, Ordering};

use sigmaos::kernel::{BuddyAllocator, Scheduler, Process, Priority};
use sigmaos::klib::paging::{SimpleVMM, VirtualMemoryManager};
use sigmaos::compatibility::{OpenRcManager, OpenRcService, OpenRcRunlevel};

/// Mock representation of x86_64 CPU Context during early boot
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    pub interrupts_enabled: bool,
    pub direction_flag_cleared: bool,
    pub gdt_base: u64,
    pub idt_base: u64,
}

impl CpuContext {
    pub const fn new() -> Self {
        CpuContext {
            interrupts_enabled: false,
            direction_flag_cleared: false,
            gdt_base: 0,
            idt_base: 0,
        }
    }
}

pub static EARLY_CPU_STATE: core::sync::atomic::AtomicUsize = AtomicUsize::new(0);

/// Early x86_64 CPU bootstrap (Simulates cli, cld, lgdt, and lidt instructions)
pub fn early_cpu_init(context: &mut CpuContext) {
    // 1. Disable interrupts (equivalent to 'cli')
    context.interrupts_enabled = false;

    // 2. Clear Direction Flag (equivalent to 'cld' for string operations)
    context.direction_flag_cleared = true;

    // 3. Load early Global Descriptor Table (equivalent to 'lgdt')
    context.gdt_base = 0x8000;

    // 4. Load Interrupt Descriptor Table (equivalent to 'lidt')
    context.idt_base = 0x9000;

    EARLY_CPU_STATE.store(1, Ordering::SeqCst);
}

/// Early Kernel memory bootstrap (sets up paging and buddy allocator)
pub fn early_memory_init(allocator: &mut BuddyAllocator, vmm: &mut SimpleVMM) {
    // 1. Setup early Identity page tables
    vmm.map_page(0x0, 0x0, false, true).unwrap();
    vmm.map_page(0x1000, 0x1000, false, true).unwrap();

    // 2. Initialize memory segments inside Buddy Allocator
    allocator.initialize_memory(0x10000, 1024 * 1024); // Allocate 1MB segment
    EARLY_CPU_STATE.store(2, Ordering::SeqCst);
}

/// The supreme x86_64 start_kernel bootstrap manager (similar to Linux's init/main.c)
pub fn start_kernel(
    context: &mut CpuContext,
    allocator: &mut BuddyAllocator,
    vmm: &mut SimpleVMM,
    scheduler: &mut Scheduler,
    openrc: &mut OpenRcManager,
) -> Result<(), &'static str> {
    // Stage 1: CPU hardware bootstrap (cli, cld, GDT, IDT)
    early_cpu_init(context);

    // Stage 2: Physical Memory paging bootstrap
    early_memory_init(allocator, vmm);

    // Stage 3: Scheduler bootstrap
    let idle_proc = Process::new(0, "idle".to_string(), Priority::Idle);
    scheduler.add_process(idle_proc);

    // Stage 4: Open early userland runlevels (OpenRC)
    let udev = OpenRcService::new("udev")
        .with_runlevel(OpenRcRunlevel::SingleUser);
    let dhcpcd = OpenRcService::new("dhcpcd")
        .with_dependency("udev")
        .with_runlevel(OpenRcRunlevel::MultiUser);

    openrc.register_service(udev);
    openrc.register_service(dhcpcd);

    // Transition runlevel to MultiUser (simulating graphical multi-user boot)
    openrc.transition_to_runlevel(OpenRcRunlevel::MultiUser)?;

    // Stage 5: Enable hardware interrupts (equivalent to 'sti')
    context.interrupts_enabled = true;
    EARLY_CPU_STATE.store(3, Ordering::SeqCst);

    Ok(())
}

use std::collections::HashMap;

/// Ubuntu/Linux-style Kernel Command Line Parser
#[derive(Debug, Clone)]
pub struct KernelCmdLineParser {
    pub params: HashMap<String, String>,
    pub flags: alloc::vec::Vec<String>,
}

impl KernelCmdLineParser {
    /// Parse a raw command-line string (e.g. "init=/bin/sh console=ttyS0 quiet boot=uefi")
    pub fn new(cmdline: &str) -> Self {
        let mut params = HashMap::new();
        let mut flags = alloc::vec::Vec::new();

        for arg in cmdline.split_whitespace() {
            if let Some(pos) = arg.find('=') {
                let key = arg[..pos].to_string();
                let val = arg[pos + 1..].to_string();
                params.insert(key, val);
            } else {
                flags.push(arg.to_string());
            }
        }

        Self { params, flags }
    }

    /// Read parameter value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Check standalone flag existence (e.g. "quiet", "debug")
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    /// Retrieve "init" binary path parameter (defaults to "/sbin/init")
    pub fn init_path(&self) -> &str {
        self.get("init").unwrap_or("/sbin/init")
    }

    /// Retrieve "console" output parameter (defaults to "tty0")
    pub fn console(&self) -> &str {
        self.get("console").unwrap_or("tty0")
    }
}

extern crate alloc;

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut context = CpuContext::new();
    let mut allocator = BuddyAllocator::new();
    let mut vmm = SimpleVMM::new();
    let mut scheduler = Scheduler::new();
    let mut openrc = OpenRcManager::new();

    let _ = start_kernel(&mut context, &mut allocator, &mut vmm, &mut scheduler, &mut openrc);

    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    let parser = KernelCmdLineParser::new("init=/bin/sh console=ttyS0 quiet boot=uefi");
    println!("Init Path: {}", parser.init_path());
    println!("Console: {}", parser.console());
    println!("Quiet flag: {}", parser.has_flag("quiet"));
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_cmdline_parsing() {
        let parser = KernelCmdLineParser::new("init=/bin/sh console=ttyS0 quiet boot=uefi debug");
        assert_eq!(parser.init_path(), "/bin/sh");
        assert_eq!(parser.console(), "ttyS0");
        assert_eq!(parser.get("boot").unwrap(), "uefi");
        assert!(parser.has_flag("quiet"));
        assert!(parser.has_flag("debug"));
        assert!(!parser.has_flag("verbose"));
    }

    #[test]
    fn test_kernel_cmdline_defaults() {
        let parser = KernelCmdLineParser::new("quiet");
        assert_eq!(parser.init_path(), "/sbin/init");
        assert_eq!(parser.console(), "tty0");
        assert!(parser.has_flag("quiet"));
    }
}
