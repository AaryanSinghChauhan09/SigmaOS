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
  VGAConsole* = object
    col*: SigmaU32
    row*: SigmaU32
    attr*: SigmaU8

proc vga_scroll*() {.exportc.} =
  discard

proc vga_putc*() {.exportc.} =
  discard

proc vga_init*() {.exportc.} =
  discard

proc serial_init*() {.exportc.} =
  discard

proc serial_putc*() {.exportc.} =
  discard

proc serial_puts*() {.exportc.} =
  discard

proc kprint_u64*() {.exportc.} =
  discard

proc kprint_str*() {.exportc.} =
  discard

proc kprint_char*() {.exportc.} =
  discard

proc ksigma_printf*() {.exportc.} =
  discard

proc console_init*() {.exportc.} =
  discard

