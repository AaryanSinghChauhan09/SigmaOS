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

// SigmaOS Interrupt & Bare-Metal Hardware Module

pub mod blog_os;
pub mod controller;
pub mod handler;

pub use blog_os::{
    ColorCode, ExceptionType, ScreenChar, TaskStateSegment, VGAColor, VGATextBuffer, GDT, IDT,
};
pub use controller::{
    APICSupport, ControllerType, IRQError, IRQHandler, IRQNumber, IRQState, InterruptController,
    SimpleIRQHandler, SimpleInterruptController,
};
pub use handler::{
    ControllerCapability as HandlerControllerCapability, HandlerCapability, HandlerType,
    InterruptDescriptor, InterruptError, InterruptHandler, InterruptHandlerInfo, InterruptManager,
    InterruptNumber, InterruptResult, InterruptStats, Priority as HandlerPriority,
    SimpleInterruptHandler, PIC,
};
