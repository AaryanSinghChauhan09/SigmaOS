## SigmaOS: SIGMA_UI_WAYLAND_H */
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  SigmaWLRect* = object

type
  SigmaWLSurface* = object
    surface_id*: SigmaI32
    geometry*: SigmaU64
    pixel_stride*: SigmaI32
    state*: SigmaU64
    z_order*: SigmaI32
    opacity*: SigmaU64
    blit*: SigmaU64
    frame_count*: SigmaU64

type
  SigmaWLCompositor* = object
    surface_count*: SigmaI32
    screen_w*: SigmaI32
    screen_h*: SigmaI32
    total_frames*: SigmaU64

proc wl_compositor_init*() {.exportc.} =
  discard

proc wl_composite_frame*() {.exportc.} =
  discard

proc wl_destroy_surface*() {.exportc.} =
  discard

proc wl_move_surface*() {.exportc.} =
  discard

