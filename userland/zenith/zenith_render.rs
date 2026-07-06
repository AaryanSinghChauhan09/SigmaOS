// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Zenith Compositor Render Primitives
//! 2D rendering on a raw framebuffer without allocations.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Framebuffer {
    pub ptr: *mut u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32, // in bytes
}

#[no_mangle]
pub unsafe extern "C" fn zenith_render_rect(
    fb: *mut Framebuffer, 
    x: u32, y: u32, w: u32, h: u32, 
    color_argb: u32
) {
    if fb.is_null() { return; }
    let fb_ref = &*fb;
    if fb_ref.ptr.is_null() { return; }
    
    let end_x = (x + w).min(fb_ref.width);
    let end_y = (y + h).min(fb_ref.height);
    
    for row in y..end_y {
        // pitch is in bytes, so divide by 4 for u32 offset
        let row_ptr = fb_ref.ptr.add((row * (fb_ref.pitch / 4)) as usize);
        for col in x..end_x {
            *row_ptr.add(col as usize) = color_argb;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn zenith_render_clear(fb: *mut Framebuffer, color_argb: u32) {
    if fb.is_null() { return; }
    let fb_ref = &*fb;
    
    zenith_render_rect(fb, 0, 0, fb_ref.width, fb_ref.height, color_argb);
}
