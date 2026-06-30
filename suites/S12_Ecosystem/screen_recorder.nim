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
  ScreenCapture* = object
    width*: SigmaU32
    height*: SigmaU32
    bpp*: SigmaU32
    frames_captured*: SigmaU64
    recording*: SigmaU64

proc screen_recorder_init*() {.exportc.} =
  discard

proc screen_recorder_stop*() {.exportc.} =
  discard

proc screen_recorder_on_refresh*() {.exportc.} =
  discard

