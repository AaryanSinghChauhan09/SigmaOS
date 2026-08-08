// SigmaOS Kernel Main Entry Point
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

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
