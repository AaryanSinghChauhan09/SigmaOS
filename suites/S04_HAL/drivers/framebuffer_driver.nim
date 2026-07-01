## SigmaOS: =============================================================================
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

proc fb_put_pixel*() {.exportc.} =
  discard

proc fb_fill_rect*() {.exportc.} =
  discard

proc fb_blit*() {.exportc.} =
  discard

proc fb_swap_buffers*() {.exportc.} =
  discard

proc fb_get_resolution*() {.exportc.} =
  discard

proc hal_framebuffer_register*() {.exportc.} =
  discard

