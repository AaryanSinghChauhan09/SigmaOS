#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS Kernel Drivers — aggregates legacy and modern driver modules
pub mod legacy;

pub use legacy::{
    E1000Driver,
    FloppyController,
    Framebuffer,
    IdeAtaController,
    // ISA bus
    IsaBus,
    // Storage
    MfmController,
    // Network
    Ne2000Driver,
    OplSynth,
    PcSpeaker,
    // Input
    Ps2Controller,
    Rtl8139Driver,
    // Audio
    SoundBlasterDriver,
    TextConsole,
    UartPort,
    UsbHidDevice,
    UsbMassStorage,
    // Display
    VgaDriver,
    VideoMode,
    // USB
    XhciController,
};
