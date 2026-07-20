/// SigmaOS Kernel Drivers — aggregates legacy and modern driver modules
pub mod legacy;

pub use legacy::{
    // ISA bus
    IsaBus,
    // Storage
    MfmController, IdeAtaController, FloppyController,
    // Input
    Ps2Controller, UartPort,
    // Display
    VgaDriver, TextConsole, Framebuffer, VideoMode,
    // Audio
    SoundBlasterDriver, OplSynth, PcSpeaker,
    // Network
    Ne2000Driver, Rtl8139Driver, E1000Driver,
    // USB
    XhciController, UsbMassStorage, UsbHidDevice,
};
