// SigmaOS Kernel Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

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
        start_kernel(&mut context, &mut allocator, &mut vmm, &mut scheduler, &mut openrc).unwrap();

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
