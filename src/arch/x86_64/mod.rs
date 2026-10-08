pub mod acpi;
pub mod boot;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod paging;
pub mod tss_ring3_user_mode;

pub fn initialize() {
    cpu::init();
    gdt::init();
    idt::init();
    paging::init();
    interrupts::enable();
    acpi::init();
    tss_ring3_user_mode::init_tss();
}
