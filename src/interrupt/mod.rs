// SigmaOS Interrupt & Bare-Metal Hardware Module

pub mod blog_os;
pub mod controller;
pub mod handler;
pub mod apic_driver;

pub use blog_os::{
    ColorCode, ExceptionType, ScreenChar, TaskStateSegment, VGAColor, VGATextBuffer, GDT, IDT,
};
pub use controller::{
    APICSupport, ControllerType, IRQError, IRQHandler, IRQNumber, IRQState, InterruptController,
    SimpleIRQHandler, SimpleInterruptController,
};
pub use handler::{
    InterruptHandler, InterruptNumber, InterruptResult, RegisterSet, SimpleInterruptHandler,
};
pub use apic_driver::{
    ApicManager, LocalApic, IoApic, InterruptDispatchTable, LocalApicId,
    VECTOR_TIMER, VECTOR_KEYBOARD, VECTOR_NETWORK, VECTOR_DISK, VECTOR_ERROR,
};
