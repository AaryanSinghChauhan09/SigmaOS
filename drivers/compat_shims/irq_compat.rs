// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/compat_shims/irq_compat.rs — IRQ API Translation Layer
//
// Provides version-aware compatibility helpers for translating different
// eras of Linux kernel IRQ handling to SigmaOS native primitives.

#![no_std]
#![allow(dead_code)]

/// Representation of a version-specific Linux IRQ handler function pointer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IrqHandlerType {
    /// Linux 2.6 style: typedef irqreturn_t (*irq_handler_t)(int, void *, struct pt_regs *);
    Legacy26(unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32),
    /// Linux 3.x to Modern style: typedef irqreturn_t (*irq_handler_t)(int, void *);
    Modern(unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32),
}

/// Dynamic wrapper for IRQ handling registration
pub struct IrqShimRegistration {
    pub irq: u32,
    pub handler: IrqHandlerType,
    pub dev_id: *mut core::ffi::c_void,
    pub dev_name: *const u8,
}

impl IrqShimRegistration {
    /// Execute the handler matching its version ABI signature.
    pub unsafe fn dispatch(&self, irq_num: i32) -> bool {
        let status = match self.handler {
            IrqHandlerType::Legacy26(handler_fn) => {
                handler_fn(irq_num, self.dev_id, core::ptr::null_mut())
            }
            IrqHandlerType::Modern(handler_fn) => {
                handler_fn(irq_num, self.dev_id)
            }
        };
        // Linux irqreturn_t: IRQ_HANDLED = 1, IRQ_NONE = 0, IRQ_WAKE_THREAD = 2
        status == 1
    }
}
