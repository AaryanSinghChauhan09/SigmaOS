pub mod acpi;
pub mod boot;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod paging;

pub fn initialize() {
    cpu::init();
    gdt::init();
    idt::init();
    paging::init();
    interrupts::enable();
    acpi::init();
}
