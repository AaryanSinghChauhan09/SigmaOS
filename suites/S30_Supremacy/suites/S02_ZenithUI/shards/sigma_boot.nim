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

proc sigma_boot_init*() {.exportc.} =
  discard

proc sigma_boot_phase_advance*() {.exportc.} =
  discard

proc sigma_boot_print_memmap*() {.exportc.} =
  discard

proc sigma_boot_entry_list*() {.exportc.} =
  discard

proc sigma_boot_entry_select*() {.exportc.} =
  discard

proc sigma_boot_attest_tpm*() {.exportc.} =
  discard

proc sigma_kexec_execute*() {.exportc.} =
  discard

proc sigma_boot_report*() {.exportc.} =
  discard

