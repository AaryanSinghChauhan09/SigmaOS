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

proc sigma_bt_inquiry_start*() {.exportc.} =
  discard

proc sigma_bt_reset*() {.exportc.} =
  discard

proc sigma_hci_rx_event*() {.exportc.} =
  discard

proc SovereignBluetooth_Init*() {.exportc.} =
  discard

