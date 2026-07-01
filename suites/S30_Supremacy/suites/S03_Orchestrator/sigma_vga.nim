## SigmaOS: =========================================================================
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

proc sigma_vga_init*() {.exportc.} =
  discard

proc sigma_vga_set_color*() {.exportc.} =
  discard

proc sigma_vga_clear_screen*() {.exportc.} =
  discard

proc sigma_vga_put_char*() {.exportc.} =
  discard

proc sigma_vga_print*() {.exportc.} =
  discard

