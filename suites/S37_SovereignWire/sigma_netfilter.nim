## SigmaOS: SIGMA_NETFILTER_H */
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
  SigmaFirewallRule* = object
    src_ip*: SigmaI32
    dst_ip*: SigmaI32
    src_port*: SigmaU64
    dst_port*: SigmaU64
    proto*: uint8
    verdict*: uint8
    hit_count*: SigmaU64
    active*: uint8

type
  SigmaFirewall* = object
    rule_count*: SigmaI32
    default_verdict*: uint8
    total_pkts*: SigmaU64
    total_dropped*: SigmaU64

proc fw_init*() {.exportc.} =
  discard

proc fw_flush*() {.exportc.} =
  discard

