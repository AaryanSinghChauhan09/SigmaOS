#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]
use std::string::String;

// SigmaOS Kernel Main Entry Point

use core::sync::atomic::{AtomicUsize, Ordering};
use std::string::ToString;

use sigmaos::arch::x86_64;
use sigmaos::compatibility::{OpenRcManager, OpenRcRunlevel, OpenRcService};
use sigmaos::kernel::{BuddyAllocator, Priority, Process, RoundRobinScheduler as Scheduler};
use sigmaos::memory::paging::SimpleVMM;

pub static EARLY_CPU_STATE: core::sync::atomic::AtomicUsize = AtomicUsize::new(0);

/// The supreme x86_64 start_kernel bootstrap manager (similar to Linux's init/main.c)
pub fn start_kernel(
    allocator: &mut BuddyAllocator,
    vmm: &mut SimpleVMM,
    scheduler: &mut Scheduler,
    openrc: &mut OpenRcManager,
) -> Result<(), &'static str> {
    // Stage 1: CPU hardware bootstrap (cli, cld, GDT, IDT, Paging)
    x86_64::initialize();
    EARLY_CPU_STATE.store(1, Ordering::SeqCst);

    // Stage 2: Physical Memory paging bootstrap
    vmm.map_page(0x0, 0x0, false, true).unwrap();
    allocator.initialize_memory(0x10000, 1024 * 1024);
    EARLY_CPU_STATE.store(2, Ordering::SeqCst);

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
    openrc.transition_to_runlevel(OpenRcRunlevel::MultiUser)?;

    EARLY_CPU_STATE.store(3, Ordering::SeqCst);
    Ok(())
}

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut allocator = BuddyAllocator::new();
    let mut vmm = SimpleVMM::new();
    let mut scheduler = Scheduler::new();
    let mut openrc = OpenRcManager::new();

    let _ = start_kernel(&mut allocator, &mut vmm, &mut scheduler, &mut openrc);

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

#[cfg(test_disabled)]
mod tests {
    use super::*;
    use sigmaos::compatibility::ServiceStatus;

    #[test]
    fn test_x86_64_start_kernel_bootstrap() {
        let mut allocator = BuddyAllocator::new();
        let mut vmm = SimpleVMM::new();
        let mut scheduler = Scheduler::new();
        let mut openrc = OpenRcManager::new();

        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 0);

        start_kernel(&mut allocator, &mut vmm, &mut scheduler, &mut openrc).unwrap();

        assert!(allocator.get_free_memory() > 0);
        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 3);
    }
}
