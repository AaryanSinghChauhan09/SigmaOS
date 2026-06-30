## SigmaOS: SIGMA_SEC_MAC_H */
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
  SigmaMACLabel* = object
    label_id*: SigmaI32

type
  SigmaMACRule* = object
    subject_id*: SigmaI32
    object_id*: SigmaI32
    permissions*: uint8
    verdict*: uint8
    hit_count*: SigmaU64

type
  SigmaMACPolicy* = object
    label_count*: SigmaI32
    rule_count*: SigmaI32
    default_verdict*: uint8

proc mac_init*() {.exportc.} =
  discard

