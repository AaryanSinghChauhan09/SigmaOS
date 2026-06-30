## SigmaOS: SIGMA_DEV_TEST_H */
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
  SigmaTest* = object
    fn*: SigmaU64
    result*: SigmaU64

type
  SigmaTestSuite* = object
    count*: SigmaI32
    passed*: SigmaI32
    failed*: SigmaI32
    skipped*: SigmaI32

proc test_suite_init*() {.exportc.} =
  discard

proc test_run_all*() {.exportc.} =
  discard

