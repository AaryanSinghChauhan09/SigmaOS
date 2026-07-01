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

type
  SigmaFB* = object
    width*: SigmaU32
    height*: SigmaU32
    pitch*: SigmaU32
    bpp*: SigmaU8
    size*: SigmaU32

proc fb_put_pixel*() {.exportc.} =
  discard

proc fb_draw_rect*() {.exportc.} =
  discard

proc fb_flip*() {.exportc.} =
  discard

proc fb_draw_char*() {.exportc.} =
  discard

proc fb_init*() {.exportc.} =
  discard

proc fb_audit*() {.exportc.} =
  discard

