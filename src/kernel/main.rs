#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]
use alloc::string::String;

// SigmaOS Kernel Main Entry Point

extern crate alloc;
use alloc::string::ToString;

use core::sync::atomic::{AtomicUsize, Ordering};

use sigmaos::compatibility::{OpenRcManager, OpenRcRunlevel, OpenRcService};
use sigmaos::kernel::{BuddyAllocator, Priority, Process, RoundRobinScheduler as Scheduler};
use sigmaos::klib::paging::{SimpleVMM, VirtualMemoryManager};

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
    let udev = OpenRcService::new("udev").with_runlevel(OpenRcRunlevel::SingleUser);
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

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut context = CpuContext::new();
    let mut allocator = BuddyAllocator::new();
    let mut vmm = SimpleVMM::new();
    let mut scheduler = Scheduler::new();
    let mut openrc = OpenRcManager::new();

    let _ = start_kernel(
        &mut context,
        &mut allocator,
        &mut vmm,
        &mut scheduler,
        &mut openrc,
    );

    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    // Hosted dummy entry point
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use sigmaos::compatibility::ServiceStatus;

    #[test]
    fn test_x86_64_start_kernel_bootstrap() {
        let mut context = CpuContext::new();
        let mut allocator = BuddyAllocator::new();
        let mut vmm = SimpleVMM::new();
        let mut scheduler = Scheduler::new();
        let mut openrc = OpenRcManager::new();

        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 0);

        // Run full early start_kernel bootstrap
        start_kernel(
            &mut context,
            &mut allocator,
            &mut vmm,
            &mut scheduler,
            &mut openrc,
        )
        .unwrap();

        // Verify context flags (cld set, sti set, gdt/idt bases loaded)
        assert!(context.interrupts_enabled);
        assert!(context.direction_flag_cleared);
        assert_eq!(context.gdt_base, 0x8000);
        assert_eq!(context.idt_base, 0x9000);

        // Verify allocator has available memory segments and paging tables are mapped
        assert!(allocator.get_free_memory() > 0);
        assert_eq!(vmm.get_physical(0x1000), Some(0x1000));

        // Verify userland OpenRC runlevel transition started services cleanly
        assert_eq!(openrc.services[0].status, ServiceStatus::Started); // udev
        assert_eq!(openrc.services[1].status, ServiceStatus::Started); // dhcpcd

        // Verify bootstrap state transition completed successfully
        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 3);
    }
}
