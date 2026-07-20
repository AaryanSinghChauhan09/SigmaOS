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
