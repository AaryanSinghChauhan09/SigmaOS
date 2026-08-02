#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

pub mod adlib_opl;
pub mod cga_mda;
pub mod floppy;
pub mod ide_ata;
pub mod isa_bus;
pub mod mfm_rll;
pub mod ne2000;
pub mod ps2_controller;
pub mod uart_8250;
pub mod usb_host;

pub use adlib_opl::{
    OplChannel, OplOperator, OplSynth, OplVersion, PcSpeaker, SbVariant, SoundBlasterDriver,
};
pub use cga_mda::{Color, Framebuffer, TextCell, TextConsole, VgaDriver, VideoMode};
pub use floppy::{FloppyController, FloppyDrive, FloppyType};
pub use ide_ata::{
    AtaCommand, AtaDeviceType, AtaDrive, AtaIdentify, IdeAtaController, TransferMode,
};
pub use isa_bus::{IsaBus, IsaDevice, IsaDmaChannel, IsaIrq, IsaResource, LpcBridge};
pub use mfm_rll::{ControllerType, DiskGeometry, MfmController, MfmDisk};
pub use ne2000::{
    E1000Driver, EthernetFrame, MacAddress, Ne2000Driver, NicDriver, NicStats, Rtl8139Driver,
};
pub use ps2_controller::{KeyCode, KeyEvent, KeyEventKind, MouseState, Ps2Controller};
pub use uart_8250::{DataBits, Parity, StopBits, Uart8250Driver, UartConfig, UartPort, UartType};
pub use usb_host::{
    HciType, UsbDevice, UsbHidDevice, UsbHostController, UsbMassStorage, UsbSpeed, XhciController,
};
