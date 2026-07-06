//! SigmaOS Magnifier (ZoomText Alternative)
//! Native magnifier reducing dependency on ZoomText, MAGic
//! Provides screen magnification, color inversion, and tracking

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaF32 = f32;
type SigmaF64 = f64;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Magnification mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MagnificationMode {
    FullScreen = 0,
    Lens = 1,
    SplitScreen = 2,
}

/// Tracking mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TrackingMode {
    None = 0,
    Mouse = 1,
    Focus = 2,
    Caret = 3,
}

/// Color mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorMode {
    Normal = 0,
    Inverted = 1,
    Grayscale = 2,
    HighContrast = 3,
}

/// Smoothing mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SmoothingMode {
    None = 0,
    Linear = 1,
    Bilinear = 2,
}

/// Magnifier region
#[repr(C)]
pub struct MagnifierRegion {
    pub x: SigmaI32,
    pub y: SigmaI32,
    pub width: SigmaU32,
    pub height: SigmaU32,
    pub zoom_level: SigmaF32,
}

/// Magnifier settings
#[repr(C)]
pub struct MagnifierSettings {
    pub mode: MagnificationMode,
    pub tracking: TrackingMode,
    pub color_mode: ColorMode,
    pub smoothing: SmoothingMode,
    pub zoom_level: SigmaF32,
    pub min_zoom: SigmaF32,
    pub max_zoom: SigmaF32,
    pub lens_width: SigmaU32,
    pub lens_height: SigmaU32,
    pub invert_colors: SigmaBool,
    pub follow_mouse: SigmaBool,
    pub follow_focus: SigmaBool,
    pub follow_caret: SigmaBool,
}

/// Magnifier
#[repr(C)]
pub struct Magnifier {
    pub settings: MagnifierSettings,
    pub region: MagnifierRegion,
    pub enabled: SigmaBool,
    pub initialized: SigmaBool,
}

static mut MAGNIFIER: Option<Magnifier> = None;

/// Initialize magnifier
#[no_mangle]
pub unsafe extern "C" fn magnifier_init() -> SigmaI32 {
    MAGNIFIER = Some(Magnifier {
        settings: MagnifierSettings {
            mode: MagnificationMode::Lens,
            tracking: TrackingMode::Mouse,
            color_mode: ColorMode::Normal,
            smoothing: SmoothingMode::Bilinear,
            zoom_level: 2.0,
            min_zoom: 1.0,
            max_zoom: 16.0,
            lens_width: 400,
            lens_height: 300,
            invert_colors: false,
            follow_mouse: true,
            follow_focus: false,
            follow_caret: false,
        },
        region: MagnifierRegion {
            x: 0,
            y: 0,
            width: 400,
            height: 300,
            zoom_level: 2.0,
        },
        enabled: false,
        initialized: false,
    });

    if let Some(mag) -> &mut MAGNIFIER {
        mag.initialized = true;
        return 0;
    }

    -1
}

/// Enable magnifier
#[no_mangle]
pub unsafe extern "C" fn magnifier_enable() -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.enabled = true;
        return 0;
    }

    -1
}

/// Disable magnifier
#[no_mangle]
pub unsafe extern "C" fn magnifier_disable() -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.enabled = false;
        return 0;
    }

    -1
}

/// Set magnification mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_mode(mode: MagnificationMode) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.mode = mode;
        return 0;
    }

    -1
}

/// Get magnification mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_mode() -> MagnificationMode {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.mode
    } else {
        MagnificationMode::Lens
    }
}

/// Set tracking mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_tracking(tracking: TrackingMode) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.tracking = tracking;
        return 0;
    }

    -1
}

/// Get tracking mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_tracking() -> TrackingMode {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.tracking
    } else {
        TrackingMode::Mouse
    }
}

/// Set color mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_color_mode(color_mode: ColorMode) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.color_mode = color_mode;
        return 0;
    }

    -1
}

/// Get color mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_color_mode() -> ColorMode {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.color_mode
    } else {
        ColorMode::Normal
    }
}

/// Set smoothing mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_smoothing(smoothing: SmoothingMode) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.smoothing = smoothing;
        return 0;
    }

    -1
}

/// Get smoothing mode
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_smoothing() -> SmoothingMode {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.smoothing
    } else {
        SmoothingMode::Bilinear
    }
}

/// Set zoom level
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_zoom(zoom: SigmaF32) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        if zoom >= mag.settings.min_zoom && zoom <= mag.settings.max_zoom {
            mag.settings.zoom_level = zoom;
            mag.region.zoom_level = zoom;
            return 0;
        }
        -1
    }
}

/// Get zoom level
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_zoom() -> SigmaF32 {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.zoom_level
    } else {
        2.0
    }
}

/// Increase zoom
#[no_mangle]
pub unsafe extern "C" fn magnifier_zoom_in() -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        let new_zoom = mag.settings.zoom_level * 1.25;
        if new_zoom <= mag.settings.max_zoom {
            mag.settings.zoom_level = new_zoom;
            mag.region.zoom_level = new_zoom;
            return 0;
        }
        -1
    }
}

/// Decrease zoom
#[no_mangle]
pub unsafe extern "C" fn magnifier_zoom_out() -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        let new_zoom = mag.settings.zoom_level / 1.25;
        if new_zoom >= mag.settings.min_zoom {
            mag.settings.zoom_level = new_zoom;
            mag.region.zoom_level = new_zoom;
            return 0;
        }
        -1
    }
}

/// Reset zoom
#[no_mangle]
pub unsafe extern "C" fn magnifier_reset_zoom() -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.zoom_level = 2.0;
        mag.region.zoom_level = 2.0;
        return 0;
    }

    -1
}

/// Set lens size
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_lens_size(width: SigmaU32, height: SigmaU32) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.lens_width = width;
        mag.settings.lens_height = height;
        mag.region.width = width;
        mag.region.height = height;
        return 0;
    }

    -1
}

/// Get lens size
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_lens_size(width: *mut SigmaU32, height: *mut SigmaU32) -> SigmaI32 {
    if MAGNIFIER.is_none() || width.is_null() || height.is_null() {
        return -1;
    }

    if let Some(mag) -> &MAGNIFIER {
        *width = mag.settings.lens_width;
        *height = mag.settings.lens_height;
        return 0;
    }

    -1
}

/// Set invert colors
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_invert_colors(invert: SigmaBool) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.invert_colors = invert;
        return 0;
    }

    -1
}

/// Get invert colors
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_invert_colors() -> SigmaBool {
    if let Some(mag) = &MAGNIFIER {
        mag.settings.invert_colors
    } else {
        false
    }
}

/// Set follow mouse
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_follow_mouse(follow: SigmaBool) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.follow_mouse = follow;
        return 0;
    }

    -1
}

/// Set follow focus
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_follow_focus(follow: SigmaBool) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.follow_focus = follow;
        return 0;
    }

    -1
}

/// Set follow caret
#[no_mangle]
pub unsafe extern "C" fn magnifier_set_follow_caret(follow: SigmaBool) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.settings.follow_caret = follow;
        return 0;
    }

    -1
}

/// Update magnifier position
#[no_mangle]
pub unsafe extern "C" fn magnifier_update_position(x: SigmaI32, y: SigmaI32) -> SigmaI32 {
    if MAGNIFIER.is_none() {
        return -1;
    }

    if let Some(mag) -> &mut MAGNIFIER {
        mag.region.x = x;
        mag.region.y = y;
        return 0;
    }

    -1
}

/// Get magnifier region
#[no_mangle]
pub unsafe extern "C" fn magnifier_get_region(region: *mut MagnifierRegion) -> SigmaI32 {
    if MAGNIFIER.is_none() || region.is_null() {
        return -1;
    }

    if let Some(mag) -> &MAGNIFIER {
        *region = mag.region;
        return 0;
    }

    -1
}

/// Check if magnifier is enabled
#[no_mangle]
pub unsafe extern "C" fn magnifier_is_enabled() -> SigmaBool {
    if let Some(mag) = &MAGNIFIER {
        mag.enabled
    } else {
        false
    }
}

/// Check if magnifier is initialized
#[no_mangle]
pub unsafe extern "C" fn magnifier_initialized() -> SigmaBool {
    if let Some(mag) = &MAGNIFIER {
        mag.initialized
    } else {
        false
    }
}

/// Helper: Copy string
unsafe fn copy_str(dest: *mut SigmaU8, src: *const SigmaU8, max_len: usize) {
    if dest.is_null() || src.is_null() {
        return;
    }
    let mut i = 0;
    while i < max_len - 1 && *src.add(i) != 0 {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    *dest.add(i) = 0;
}

/// Helper: Get string length
unsafe fn str_len(s: *const SigmaU8) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 512 {
        len += 1;
    }
    len
}
