pub mod firmware;
pub mod multiboot2;

pub use firmware::{FirmwareInterface, BootLoader, BootParams, SetupHeader, Initramfs, KernelCommandLine, FirmwareMemoryMapEntry, FirmwareInfo, AcpiTable, SmpInfo, BootError};
pub use multiboot2::{
    Multiboot2Header, Multiboot2BootLoader, ParsedMbi, MbiFramebuffer, MmapEntry as Multiboot2MmapEntry, MbiModule,
    parse_mbi, validate_header,
};