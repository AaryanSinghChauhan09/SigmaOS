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
  MockCallEntry* = object
    arg0*: SigmaU64
    timestamp*: SigmaU64

proc mock_log*() {.exportc.} =
  discard

proc mock_hal_set_fail_mask*() {.exportc.} =
  discard

proc mock_hal_clear_fails*() {.exportc.} =
  discard

proc mock_display_put_pixel*() {.exportc.} =
  discard

proc mock_display_fill_rect*() {.exportc.} =
  discard

proc mock_display_blit*() {.exportc.} =
  discard

proc mock_display_swap*() {.exportc.} =
  discard

proc mock_display_get_res*() {.exportc.} =
  discard

proc mock_timer_sleep*() {.exportc.} =
  discard

proc mock_hal_advance_ticks*() {.exportc.} =
  discard

proc mock_serial_write_byte*() {.exportc.} =
  discard

proc mock_serial_write_str*() {.exportc.} =
  discard

proc mock_hal_register_all*() {.exportc.} =
  discard

